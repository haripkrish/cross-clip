use libp2p::identity::Keypair;
use libp2p::{gossipsub, mdns, PeerId, SwarmBuilder};
use libp2p::gossipsub::{Behaviour, Config};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use std::error::Error;
use std::time::Duration;
use tokio::{io, select};

fn create_new_key_pair() -> Keypair {
    let local_keypair: Keypair = Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_keypair.public());
    println!("Local peer id: {:?}", local_peer_id);
    local_keypair
}

pub(crate) fn run_swarm() {
    // Create a new key pair
    let keypair = create_new_key_pair();

    let mdns = match mdns::tokio::Behaviour::new(mdns::Config::default(), keypair.public().to_peer_id()) {
        Ok(_) => {
            println!("OK MDNS");
        }
        Err(_) => {
            println!("ERROR MDNS");
        }
    };

    let gossipsub: Behaviour = match Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair.clone()),
        get_gossipsub_config().unwrap_or_else(|err| panic!("Failed to build gossipsub config: {:?}", err)),
    ) {
        Ok(behaviour) => behaviour,
        Err(err) => panic!("Error initializing gossipsub behaviour: {:?}", err),
    };
}

fn get_gossipsub_config() -> Result<Config, Box<dyn Error>> {
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;
    Ok(gossipsub_config)
}
