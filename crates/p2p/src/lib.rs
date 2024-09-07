use futures::stream::StreamExt;
use libp2p::{
    gossipsub::{self, Message, MessageId},
    kad, noise, swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{sync::mpsc, io, select};
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

struct GossipMessage {
    topic: String,
    message: String,
}

pub struct KarakP2P {
    swarm: Option<Swarm<KarakP2PBehaviour>>,
    message_receiver: mpsc::Receiver<GossipMessage>,
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

    fn start_server(
        &mut self,
        topic: &str,
        listen_addr: Multiaddr,
        bootstrap_addrs: Vec<P2PNode>,
    ) -> Result<&mut Self, Box<dyn Error>> {
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
            println!("Adding peer: {:?}, {:?}", peer.peer_id, peer.address);
            self.swarm
                .as_mut()
                .expect("swarm not defined")
                .behaviour_mut()
                .kademlia
                .add_address(&peer.peer_id, peer.address.clone());
        }
        Ok(self)
    }

    async fn start_listening(
        &mut self,
        on_incoming_message: fn(PeerId, MessageId, Message),
    ) -> Result<(), Box<dyn Error>> {
        loop {
            select! {
                Some(gossip_message) = self.message_receiver.recv() => {
                    self.publish_message(gossip_message.topic.as_str(), gossip_message.message.as_str()).unwrap();
                }
                event = self.swarm.as_mut().expect("swarm not defined").select_next_some() => match event {
                    SwarmEvent::Behaviour(KarakP2PBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => on_incoming_message(peer_id, id, message)                        ,
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    }
                    _ => {}
                }
            }
        }
    }

    fn publish_message(&mut self, topic: &str, message: &str) -> Result<(), Box<dyn Error>> {
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

    use futures::{FutureExt, TryFutureExt};
    use io::Interest;
    use tokio::sync::oneshot;

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Intended panic for testing")]
    async fn test_entire_flow() -> () {
        let (message_sender_one, message_receiver_one) = mpsc::channel::<GossipMessage>(100);
        let (message_sender_two, message_receiver_two) = mpsc::channel::<GossipMessage>(100);
        let (tx, rx) = oneshot::channel::<PeerId>();

        let handle1 = tokio::spawn(async move {
            let mut karak_p2p = KarakP2P {
                swarm: None,
                message_receiver: message_receiver_one,
            };
            karak_p2p.create_swarm().unwrap();
            let peer_id = karak_p2p.peer_id().clone();
            let karak_p2p_server = karak_p2p
                .start_server(
                    "test",
                    "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
                    vec![],
                )
                .unwrap();
            tx.send(peer_id).unwrap();
            karak_p2p_server
                .start_listening(|_peer_id, _id, message| {
                    println!(
                        "This is the incoming message: {:?}",
                        String::from_utf8_lossy(&message.data)
                    );
                    panic!("Intended panic for testing")
                })
                .await
                .unwrap();
        });

        let handle2 = tokio::spawn(async move {
            let mut karak_p2p = KarakP2P {
                swarm: None,
                message_receiver: message_receiver_two,
            };
            karak_p2p.create_swarm().unwrap();
            let peer_id = match rx.await {
                Ok(v) => v,
                Err(_) => panic!(),
            };
            karak_p2p
                .start_server(
                    "test",
                    "/ip4/127.0.0.1/tcp/8136".parse::<Multiaddr>().unwrap(),
                    vec![P2PNode {
                        peer_id: peer_id,
                        address: "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
                    }],
                )
                .unwrap()
                .start_listening(|_peer_id, _id, message| {
                    println!(
                        "This is the incoming message: {:?}",
                        String::from_utf8_lossy(&message.data)
                    );
                    panic!()
                })
                .await
                .unwrap();
        });
        let handle3 = tokio::spawn(async move {
         // thread::sleep(Duration::from_secs(20));
            tokio::time::sleep(Duration::from_secs(10)).await;
            message_sender_two
                .send(GossipMessage {
                    topic: "test".to_string(),
                    message: "test message".to_string(),
                })
                .await
                .unwrap();
        });

        let _ = tokio::join!(handle1 , handle3);
        panic!("Intended panic for testing")
    }
}
