use libp2p::{gossipsub, mdns};
use libp2p::swarm::NetworkBehaviour;
use libp2p::gossipsub::Behaviour;

#[derive(NetworkBehaviour)]
#[behaviour(event_name = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub gossipsub: Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    // pingBehaviour: ping::Behaviour,
}