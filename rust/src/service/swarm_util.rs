use std::time::Duration;

use libp2p::{gossipsub, mdns, Swarm, SwarmBuilder};
use libp2p::gossipsub::{Behaviour, Config};
use tokio::io;

use crate::schema::message::InputMessage;
use crate::schema::swarm_model::MyBehaviour;
use crate::service::key_pair_util::{generate_new_key_pair, get_keypair_from_mnemonic_str, get_peer_id};

pub fn publish_message(topic_name: &str, message: &InputMessage, gossip_behaviour: &mut Behaviour) -> bool {
    let topic = gossipsub::IdentTopic::new(topic_name);
    let message_str = serde_json::to_string(&message).unwrap();

    let message_id = gossip_behaviour
        .publish(topic.clone(), message_str.as_bytes())
        .unwrap_or_else(
            |err| panic!("Failed to send message {:?}", err)
        );

    println!("message sent {}", message_id);
    true
}

pub fn initialize_swarm(mnemonic_string: &str) -> (Swarm<MyBehaviour>, String) {
    let mut keypair = get_keypair_from_mnemonic_str(mnemonic_string);
    let topic_id = get_peer_id(&keypair).to_string();

    keypair = generate_new_key_pair();
    println!("Local Keypair {}", get_peer_id(&keypair).to_string());

    let mdns = mdns::tokio::Behaviour::new(
        mdns::Config::default(), keypair.public().to_peer_id(),
    ).unwrap_or_else(|err| panic!("Failed to build MDNS config: {:?}", err));

    let gossipsub: Behaviour = Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair.clone()),
        get_gossipsub_config(),
    ).unwrap_or_else(|err| panic!("Failed to build gossipsub behaviour: {:?}", err));

    let mut swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_quic()
        .with_behaviour(|_local_keypair| {
            Ok(MyBehaviour { gossipsub, mdns })
        })
        .unwrap_or_else(|err| panic!("Failed At the behaviour : {:?}", err))
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap_or_else(|err| panic!("Failed to build QUIC config: {:?}", err))).unwrap_or_else(|err| panic!("Failed to build SWARM RUN  config: {:?}", err));

    subscribe_to_topic(&topic_id, &mut swarm);

    (swarm, topic_id)
}

fn subscribe_to_topic(topic_id: &String, swarm: &mut Swarm<MyBehaviour>) {
    let topic = gossipsub::IdentTopic::new(topic_id);
    println!("Topic Name {}", topic.to_string());
    swarm
        .behaviour_mut().gossipsub
        .subscribe(&topic)
        .unwrap_or_else(
            |err| {
                panic!("Failed to subscribe: {:?}", err)
            }
        );
    println!("Subscribed topics: {:?}", swarm.behaviour_mut().gossipsub.topics().count());
}

fn get_gossipsub_config() -> Config {
    gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))
        .unwrap_or_else(|err| panic!("Failed to build GOSSIP config: {:?}", err))
}