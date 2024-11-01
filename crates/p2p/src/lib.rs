use futures::stream::StreamExt;
use libp2p::{
    gossipsub::{self, Message, MessageId},
    kad, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, TransportError,
};
use std::{
    collections::hash_map::DefaultHasher,
    future::Future,
    hash::{Hash, Hasher},
    time::Duration,
};
use thiserror::Error;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};

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
    #[error("builder error")]
    BuilderError,
}

// We create a custom network behaviour that combines Gossipsub and Kademlia.
#[derive(NetworkBehaviour)]
pub struct KarakP2PBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

pub struct P2PNode {
    pub peer_id: PeerId,
    pub address: Multiaddr,
}

pub struct GossipMessage<M: AsRef<[u8]>> {
    topic: String,
    message: M,
}

impl<M: AsRef<[u8]>> GossipMessage<M> {
    pub fn new(topic: String, message: M) -> Self {
        GossipMessage { topic, message }
    }
}

pub struct KarakP2P<M: AsRef<[u8]>> {
    swarm: Swarm<KarakP2PBehaviour>,
    termination_receiver: oneshot::Receiver<()>,
    message_receiver: mpsc::Receiver<GossipMessage<M>>,
}

impl<M: AsRef<[u8]>> KarakP2P<M> {
    pub fn create_and_start_swarm(
        topic: &str,
        listen_addr: Multiaddr,
        bootstrap_addrs: Vec<P2PNode>,
        termination_receiver: oneshot::Receiver<()>,
        message_receiver: mpsc::Receiver<GossipMessage<M>>,
        idle_timeout_duration: u64,
    ) -> Result<Self, KarakP2PError> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
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
                    .heartbeat_interval(Duration::from_secs(1)) // More frequent heartbeats
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .mesh_n_low(2) // Minimum number of peers to maintain in mesh
                    .mesh_n(6) // Target number of peers to keep in mesh
                    .mesh_n_high(12) // Maximum number of peers in mesh
                    .mesh_outbound_min(2) // Minimum outbound peers in mesh
                    .flood_publish(true) // Ensure messages are flooded to all peers
                    .history_length(10) // Keep track of more messages
                    .history_gossip(3) // Gossip about more messages
                    .build()
                    .map_err(|_| KarakP2PError::BuilderError)?;

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
            .map_err(|_| KarakP2PError::BehaviourError)?
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
            tracing::info!("Adding peer: {:?}, {:?}", peer.peer_id, peer.address);
            swarm
                .behaviour_mut()
                .kademlia
                .add_address(&peer.peer_id, peer.address.clone());
        }

        tracing::info!("Swarm peer id: {:?}", swarm.local_peer_id());

        Ok(KarakP2P {
            swarm,
            termination_receiver,
            message_receiver,
        })
    }

    pub async fn start_listening<F, Fut>(
        &mut self,
        on_incoming_message: F,
    ) -> Result<(), KarakP2PError>
    where
        F: Fn(PeerId, MessageId, Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send,
    {
        loop {
            select! {
                Ok(_) = &mut self.termination_receiver => {
                    tracing::info!("Termination message received");
                }
                Some(gossip_message) = self.message_receiver.recv()=> {
                    self.publish_message(&gossip_message.topic, gossip_message.message).unwrap_or_else(|e| {
                        tracing::error!("Failed to publish message: {:?}", e);
                    });
                }
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(KarakP2PBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => on_incoming_message(peer_id, id, message).await,
                    SwarmEvent::NewListenAddr { address, .. } => {
                        tracing::info!("Local node is listening on {address}");
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        tracing::info!("Connection established with peer: {:?}", peer_id);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn publish_message(&mut self, topic: &str, message: M) -> Result<(), KarakP2PError> {
        let topic_hash = gossipsub::IdentTopic::new(topic);
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic_hash, message.as_ref())?;
        Ok(())
    }

    pub fn peer_id(&mut self) -> PeerId {
        self.swarm.local_peer_id().to_owned()
    }
}