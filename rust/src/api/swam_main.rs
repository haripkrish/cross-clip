use libp2p::identity::Keypair;
use libp2p::{gossipsub, mdns, PeerId, SwarmBuilder};
use libp2p::gossipsub::{Behaviour, Config};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use std::error::Error;
use std::time::Duration;
use tokio::{io, select};


#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: Behaviour,
    mdns: mdns::tokio::Behaviour,
    // pingBehaviour: ping::Behaviour,
}

fn create_new_key_pair() -> Keypair {
    let local_keypair: Keypair = Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_keypair.public());
    println!("Local peer id: {:?}", local_peer_id);
    local_keypair
}

#[tokio::main]
pub(crate) async fn run_swarm() -> Result<(), Box<dyn Error>> {
    // Create a new key pair
    let keypair = create_new_key_pair();

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), keypair.public().to_peer_id()).unwrap_or_else(|err| panic!("Failed to build MDNS config: {:?}", err));

    let gossipsub: Behaviour = Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair.clone()),
        get_gossipsub_config().unwrap_or_else(|err| panic!("Failed to build gossipsub config: {:?}", err)),
    ).unwrap_or_else(|err| panic!("Failed to build gossipsub config: {:?}", err));

    let mut swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_quic()
        .with_behaviour(|_local_keypair| {
            Ok(MyBehaviour { gossipsub, mdns })
        }).unwrap_or_else(|err| panic!("Failed At the behaviour : {:?}", err))
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap_or_else(|err| panic!("Failed to build QUIC config: {:?}", err))).unwrap_or_else(|err| panic!("Failed to build SWAM RUN  config: {:?}", err));

    Ok(())
}

fn get_gossipsub_config() -> Result<Config, Box<dyn Error>> {
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;
    Ok(gossipsub_config)
}
