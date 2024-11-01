// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![doc = include_str!("../README.md")]

use futures::stream::StreamExt;
use libp2p::gossipsub::{Message, MessageId};
use libp2p::multiaddr::Protocol;
use libp2p::{
    gossipsub, kad, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux,
};
use libp2p::{Multiaddr, PeerId};
use std::borrow::BorrowMut;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, select, test};
use tracing_subscriber::EnvFilter;

// We create a custom network behaviour that combines Gossipsub and Kademlia.
#[derive(NetworkBehaviour)]
struct KarakP2PBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

struct P2PNode {
    pub peer_id: PeerId,
    pub address: Multiaddr,
}

struct UserDefined;

trait P2PUserDefined {
    fn on_incoming_message(&self, propagation_source: PeerId, id: MessageId, message: Message);
}

pub struct KarakP2P {
    swarm: Option<Swarm<KarakP2PBehaviour>>,
}

impl KarakP2P {
    fn create_swarm(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        self.swarm = Some(
            libp2p::SwarmBuilder::with_new_identity()
                .with_tokio()
                .with_tcp(
                    tcp::Config::default(),
                    noise::Config::new,
                    yamux::Config::default,
                )?
                .with_dns()?
                .with_behaviour(|key| {
                    // To content-address message, we can take the hash of message and use it as an ID.
                    let message_id_fn = |message: &gossipsub::Message| {
                        let mut s = DefaultHasher::new();
                        message.data.hash(&mut s);
                        gossipsub::MessageId::from(s.finish().to_string())
                    };

                    // Set a custom gossipsub configuration
                    let gossipsub_config = gossipsub::ConfigBuilder::default()
                        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                        .build()
                        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

                    // build a gossipsub network behaviour
                    let gossipsub = gossipsub::Behaviour::new(
                        gossipsub::MessageAuthenticity::Signed(key.clone()),
                        gossipsub_config,
                    )?;

                    println!("peer id: {}", &key.public().to_peer_id());

                    let store = kad::store::MemoryStore::new(key.public().to_peer_id());
                    let kademlia = kad::Behaviour::new(key.public().to_peer_id(), store);

                    Ok(KarakP2PBehaviour {
                        gossipsub,
                        kademlia,
                    })
                })?
                .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
                .build(),
        );

        Ok(self)
    }

    async fn start_listening<T: P2PUserDefined>(
        &mut self,
        topic: &str,
        listen_addr: Multiaddr,
        bootstrap_addrs: Vec<P2PNode>,
        user_defined_trait_impl: &T,
    ) -> Result<(), Box<dyn Error>> {
        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new(topic);
        // subscribes to our topic
        self.swarm
            .as_mut()
            .expect("swarm not defined")
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)?;

        // Listen on all interfaces and whatever port the OS assigns
        self.swarm
            .as_mut()
            .expect("swarm not defined")
            .listen_on(listen_addr)?;

        for peer in &bootstrap_addrs {
            self.swarm
                .as_mut()
                .expect("swarm not defined")
                .behaviour_mut()
                .kademlia
                .add_address(&peer.peer_id, peer.address.clone());
        }

        loop {
            select! {
                event = self.swarm.as_mut().expect("swarm not defined").select_next_some() => match event {
                    SwarmEvent::Behaviour(KarakP2PBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => user_defined_trait_impl.on_incoming_message(peer_id, id, message)                        ,
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    }
                    _ => {}
                }
            }
        }
    }

    fn publish_message(
        &mut self,
        topic: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error>> {
        let topic_hash = gossipsub::IdentTopic::new(topic);
        self.swarm
            .as_mut()
            .expect("swarm not defined")
            .behaviour_mut()
            .gossipsub
            .publish(topic_hash, message.as_bytes())?;
        Ok(())
    }

    fn peer_id(&mut self) -> PeerId {
        self.swarm
            .as_mut()
            .expect("swarm not defined")
            .local_peer_id()
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    impl P2PUserDefined for UserDefined {
        fn on_incoming_message(&self, propagation_source: PeerId, id: MessageId, message: Message) {
            println!(
                "This is the incoming message: {:?}",
                String::from_utf8_lossy(&message.data)
            )
        }
    }

    #[tokio::test]
    async fn test_entire_flow() -> Result<(), Box<dyn Error>> {
        tokio::spawn(async {
        let user_defined = UserDefined;
        let mut karak_p2p = KarakP2P { swarm: None };
        karak_p2p.create_swarm().unwrap();
        let peer_id = karak_p2p.peer_id().clone();
            karak_p2p
                .start_listening(
                    "test",
                    "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
                    vec![],
                    &user_defined,
                )
                .await.unwrap();
        });

        tokio::spawn(async {
            let user_defined = UserDefined;
            let mut karak_p2p = KarakP2P { swarm: None };
            karak_p2p.create_swarm().unwrap();
            let peer_id = karak_p2p.peer_id().clone();
                karak_p2p
                    .start_listening(
                        "test",
                        "/ip4/127.0.0.1/tcp/8135".parse::<Multiaddr>().unwrap(),
                        vec![P2PNode {
                            peer_id: peer_id.clone(),
                            address: "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
                        }],
                        &user_defined,
                    )
                    .await.unwrap();

            karak_p2p.publish_message("test", "this is a test").unwrap()
            });

        let user_defined = UserDefined;
        let mut karak_p2p = KarakP2P { swarm: None };
        karak_p2p.create_swarm().unwrap();
        let peer_id = karak_p2p.peer_id().clone();
            karak_p2p
                .start_listening(
                    "test",
                    "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
                    vec![],
                    &user_defined,
                )
                .await.unwrap();
        karak_p2p.publish_message("test", "this is a test").unwrap();

            thread::sleep(Duration::from_secs(15));
        Ok(())
    }
}
