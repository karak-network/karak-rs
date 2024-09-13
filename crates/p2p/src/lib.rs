use futures::stream::StreamExt;
use libp2p::{
    gossipsub::{self, Message, MessageId},
    kad, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, TransportError,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use thiserror::Error;
use tokio::{io, select, sync::{mpsc, oneshot}};

#[derive(Debug, Error)]
pub enum KarakP2PError {
    #[error("Failed to create swarm")]
    SwarmCreationError,
    #[error("libp2p noise failed")]
    NoiseError(#[from] noise::Error),
    #[error("libp2p dns failed")]
    TransportError(#[from] std::io::Error),
    #[error("libp2p behaviour failed")]
    BehaviourError,
    #[error("libp2p subscription failed")]
    SubscriptionError(#[from] libp2p::gossipsub::SubscriptionError),
    #[error("libp2p listen failed")]
    ListenError(#[from] TransportError<std::io::Error>),
    #[error("libp2p publish failed")]
    PublishError(#[from] libp2p::gossipsub::PublishError),
}

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

struct GossipMessage<M: AsRef<[u8]>> {
    topic: String,
    message: M,
}

pub struct KarakP2P {
    swarm: Swarm<KarakP2PBehaviour>,
    termination_receiver: oneshot::Receiver<()>,
}

impl KarakP2P {
    fn create_and_start_swarm<M: AsRef<[u8]>>(
        topic: &str,
        listen_addr: Multiaddr,
        bootstrap_addrs: Vec<P2PNode>,
        message_receiver: oneshot::Receiver<()>,
        idle_timeout_duration: u64,
    ) -> Result<Self, KarakP2PError> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(KarakP2PError::NoiseError)?
            .with_dns()
            .map_err(KarakP2PError::TransportError)?
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
            })
            .map_err(|_private_error| KarakP2PError::BehaviourError)?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(Duration::from_secs(idle_timeout_duration))
            })
            .build();

        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new(topic);
        // subscribes to our topic
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on(listen_addr)?;

        for peer in &bootstrap_addrs {
            println!("Adding peer: {:?}, {:?}", peer.peer_id, peer.address);
            swarm
                .behaviour_mut()
                .kademlia
                .add_address(&peer.peer_id, peer.address.clone());
        }

        Ok(KarakP2P {
            swarm,
            termination_receiver: message_receiver,
        })
    }

    async fn start_listening(
        &mut self,
        on_incoming_message: fn(PeerId, MessageId, Message),
        extra_fields: Option<fn()>,
    ) -> Result<(), KarakP2PError> {
        loop {
            select! {
                Ok(_) = &mut self.termination_receiver => {
                    panic!("Termination message received");
                }
                event = self.swarm.select_next_some() => match event {
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

            if let Some(mut fields) = extra_fields {
                fields();
            }
        }
    }

    fn publish_message<M: AsRef<[u8]>>(&mut self, topic: &str, message: M) -> Result<(), KarakP2PError> {
        let topic_hash = gossipsub::IdentTopic::new(topic);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic_hash, message.as_ref())?;
        Ok(())
    }

    fn peer_id(&mut self) -> PeerId {
        self.swarm.local_peer_id().to_owned()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::thread;

//     use futures::{FutureExt, TryFutureExt};
//     use io::Interest;
//     use tokio::sync::oneshot;

//     use super::*;

//     #[tokio::test]
//     async fn test_entire_flow() -> () {
//         let (message_sender_one, message_receiver_one) =
//             mpsc::channel::<GossipMessage<String>>(100);
//         let (message_sender_two, message_receiver_two) =
//             mpsc::channel::<GossipMessage<String>>(100);
//         let (tx, rx) = oneshot::channel::<PeerId>();

//         let handle1 = tokio::spawn(async move {
//             let mut karak_p2p_server = KarakP2P::create_and_start_swarm(
//                 "test",
//                 "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
//                 vec![],
//                 message_receiver_one,
//                 60,
//             )
//             .unwrap();

//             let peer_id = karak_p2p_server.peer_id().clone();
//             tx.send(peer_id).unwrap();

//             karak_p2p_server
//                 .start_listening(|_peer_id, _id, message| {
//                     println!(
//                         "This is the incoming message: {}",
//                         String::from_utf8_lossy(&message.data)
//                     );
//                     panic!("Intended panic for testing")
//                 })
//                 .await
//                 .unwrap();
//         });

//         let handle2 = tokio::spawn(async move {
//             let peer_id = match rx.await {
//                 Ok(v) => v,
//                 Err(_) => panic!(),
//             };
//             KarakP2P::create_and_start_swarm(
//                 "test",
//                 "/ip4/127.0.0.1/tcp/8136".parse::<Multiaddr>().unwrap(),
//                 vec![P2PNode {
//                     peer_id: peer_id,
//                     address: "/ip4/127.0.0.1/tcp/8134".parse::<Multiaddr>().unwrap(),
//                 }],
//                 message_receiver_two,
//                 60,
//             )
//             .unwrap()
//             .start_listening(|_peer_id, _id, message| {
//                 println!(
//                     "This is the incoming message: {}",
//                     String::from_utf8_lossy(&message.data)
//                 );
//                 panic!()
//             })
//             .await
//             .unwrap();
//         });
//         let handle3 = tokio::spawn(async move {
//             // thread::sleep(Duration::from_secs(20));
//             tokio::time::sleep(Duration::from_secs(10)).await;
//             message_sender_two
//                 .send(GossipMessage {
//                     topic: "test".to_string(),
//                     message: "test message".to_string(),
//                 })
//                 .await
//                 .unwrap();
//         });

//         let _ = tokio::join!(handle1, handle3);
//     }
// }
