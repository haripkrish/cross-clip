use std::{thread::sleep, time::Duration};
use std::collections::HashMap;
use std::convert::identity;
use std::io::Bytes;
use std::str::FromStr;
use std::sync::{RwLock, TryLockResult};

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use futures::StreamExt;
use libp2p::{gossipsub, identity, mdns, PeerId, Swarm};
use libp2p::identity::Keypair;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use log::{debug, error, info};
use tokio::{io, select};

use crate::api::swam_main;
use crate::frb_generated::StreamSink;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello yo1aa, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_1() -> String {
    format!("test")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[derive(Debug, Clone)]
pub struct CustomResponseEvent {
    pub data: HashMap<String, Vec<HashMap<String, String>>>,
}

impl CustomResponseEvent {
    pub fn new() -> Self {
        CustomResponseEvent {
            data: HashMap::new()
        }
    }

    pub(crate) fn set_note(&mut self, user_device_id: &str, note_header: &str, note_value: &str) {
        let note_entry = HashMap::from([(note_header.to_string(), note_value.to_string())]);

        let entry = self.data.entry(user_device_id.to_string()).or_insert_with(Vec::new);
        entry.push(note_entry);
    }

    fn get_note(&self, user_device_id: &str) -> Option<&Vec<HashMap<String, String>>> {
        self.data.get(user_device_id)
    }
}


fn main1() -> String {
    println!("Test note storage");

    let mut note_storage = CustomResponseEvent::new();

    println!("Get note by user device ID");
    if let Some(notes) = note_storage.get_note("asd") {
        for note in notes {
            for (k, v) in note {
                println!("Note header: {}, Notes: {}", k, v)
            }
        }
    }

    println!("Set note by user device ID");
    note_storage.set_note("Hari_device1", "Note_header1", "Note_value1");
    note_storage.set_note("Hari_device1", "Note_header2", "Note_value2");
    note_storage.set_note("Hari_device2", "Note_header1", "Note_value1");
    note_storage.set_note("Karthik_device1", "Note_header1", "Note_value1");

    println!("Get note by user device ID");
    let user_device_id_test = "Hari_device2";

    if let Some(notes) = note_storage.get_note(user_device_id_test) {
        println!("Note for {}", user_device_id_test);
        for note in notes {
            for (k, v) in note {
                println!("Note header: {}, Notes: {}", k, v)
            }
        }
        println!("\n");
    }

    println!("Note storage: {:?}", note_storage);
    format!("test")
}


#[flutter_rust_bridge::frb(sync)]
pub fn test_4() -> String {
    println!("Test note storage");

    let mut note_storage = CustomResponseEvent::new();
    println!("Set note by user device ID");
    note_storage.set_note("Hari_device1", "Note_header1", "Note_value1");
    println!("Get note by user device ID");
    let user_device_id_test = "Hari_device1";

    if let Some(notes) = note_storage.get_note(user_device_id_test) {
        println!("Note for {}", user_device_id_test);
        for note in notes {
            for (k, v) in note {
                println!("Note header: {}, Notes: {}", k, v)
            }
        }
        println!("\n");
    }
    format!("test2")
}


const ONE_SECOND: Duration = Duration::from_secs(2);
static SWARM_EVENT_STREAM: RwLock<Option<StreamSink<CustomResponseEvent>>> = RwLock::new(None);
pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    println!("tick called");
    let mut ticks = 0;
    loop {
        sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}

fn generate_new_key_pair() -> Keypair {
    let local_keypair: Keypair = Keypair::generate_ed25519();
    local_keypair
}
fn generate_new_mnemonic() -> Mnemonic {
    let mnemonic = Mnemonic::new(MnemonicType::Words15, Default::default());
    println!("{}", mnemonic);
    mnemonic
}
fn get_mnemonic_from_str(mnemonic_string: &str) -> Mnemonic {
    Mnemonic::from_phrase(mnemonic_string, Language::English).unwrap()
}
fn get_keypair_from_mnemonic_str(mnemonic_string: &str) -> Keypair {
    let mnemonic = Mnemonic::from_phrase(mnemonic_string, Default::default()).unwrap();
    get_keypair_from_mnemonic(mnemonic)
}
fn get_keypair_from_mnemonic(mnemonic: Mnemonic) -> Keypair {
    let mut entropy_bytes = [0u8; 32];
    entropy_bytes[..mnemonic.entropy().len()].copy_from_slice(&mnemonic.entropy());
    Keypair::ed25519_from_bytes(entropy_bytes).unwrap()
}
fn get_peer_id(keypair: &Keypair) -> PeerId {
    let local_peer_id: PeerId = PeerId::from(keypair.public());
    local_peer_id
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_restoring_keypair() {
        let mnemonic = generate_new_mnemonic();
        let mnemonic_string = mnemonic.to_string();
        let keypair = get_keypair_from_mnemonic(mnemonic);
        let restored_keypair = get_keypair_from_mnemonic_str(&mnemonic_string);
        assert_eq!(keypair.public(), restored_keypair.public());
    }
    #[test]
    fn test_mnemonic_to_keypair() {
        let mnemonic_string = "actress barely uniform warm task parade hamster brave patient airport inform seek cigar shop try";
        let mnemonic = Mnemonic::from_phrase(mnemonic_string, Default::default()).unwrap();
        let keypair = get_keypair_from_mnemonic(mnemonic);
        let peer_id = keypair.public().to_peer_id();
        assert_eq!(peer_id.to_string(), "12D3KooWLvqSPsvygzZpUGW9NucmwZZVkhEQhn5Wz9hv5tRrEzhh");
    }
}

#[tokio::main]
pub async fn run_app(s: StreamSink<CustomResponseEvent>) {
    println!("Starting Application RUN_APP");

    let mut note_storage = CustomResponseEvent::new();
    let mut ticks = 0;

    // let mnemonic = generate_new_mnemonic();
    let mnemonic = get_mnemonic_from_str(&"color cigar trouble domain floor math card festival hammer safe govern cute strong common patient");
    let keypair = get_keypair_from_mnemonic_str(&*mnemonic.to_string());
    let peer_id = get_peer_id(&keypair);

    let topic = gossipsub::IdentTopic::new(peer_id.to_string());
    println!("Topic Name {}", topic.to_string());

    let new_keypair = generate_new_key_pair();
    println!("Local Keypair {}", get_peer_id(&new_keypair).to_string());

    let mdns = mdns::tokio::Behaviour::new(
        mdns::Config::default(), new_keypair.public().to_peer_id(),
    ).unwrap_or_else(|err| panic!("Failed to build MDNS config: {:?}", err));

    let gossipsub: Behaviour = Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(new_keypair.clone()),
        get_gossipsub_config().unwrap_or_else(|err| panic!("Failed to build gossipsub config: {:?}", err)),
    ).unwrap_or_else(|err| panic!("Failed to build gossipsub behaviour: {:?}", err));

    let mut swarm = SwarmBuilder::with_existing_identity(new_keypair.clone())
        .with_tokio()
        .with_quic()
        .with_behaviour(|_local_keypair| {
            Ok(MyBehaviour { gossipsub, mdns })
        }).unwrap_or_else(|err| panic!("Failed At the behaviour : {:?}", err))
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap_or_else(|err| panic!("Failed to build QUIC config: {:?}", err))).unwrap_or_else(|err| panic!("Failed to build SWARM RUN  config: {:?}", err));

    let x = swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap_or_else(|err| panic!("Failed to subscribe: {:?}", err));

    loop {
        select! {
            event = swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                        let line = "THIS IS A TEST MESSAGE";
                        let message = InputMessage::new(line.to_string());
                        // println!("{:?}", message);
                        // publish_message(peer_id.to_string(), &message, &mut swarm.behaviour_mut().gossipsub);
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                           swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message })) => {
                            let message_struct:InputMessage = serde_json::from_slice(&message.data).unwrap();
                        println!(
                                "Got message: '{}' and timestamp: '{}'\n id: {id} \n peer: {peer_id}",
                                &message_struct.message, &message_struct.timestamp
                            );
                    },
                    _ => {}
                }
    }

        // loop {
        //     s.add(note_storage.clone());
        //     sleep(ONE_SECOND);
        //     if 100 == i32::MAX {
        //         break;
        //     }
        //     println!("Set note by user device ID");
        //     note_storage.set_note("Hari_device1", "Note_header1", &format!("{}{}", "", ticks));
        //     ticks += 1;
        // }
    }
}


use libp2p::identity::{PublicKey};
use libp2p::{multihash, SwarmBuilder};
use libp2p::gossipsub::{Behaviour, Config};
use std::error::Error;
use hex::encode;
use libp2p::identity::KeyType::Ed25519;
use libp2p::mdns::tokio::Tokio;
use libp2p::swarm::FromSwarm::ConnectionEstablished;
use crate::schema::message::InputMessage;

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    gossipsub: Behaviour,
    mdns: mdns::tokio::Behaviour,
    // pingBehaviour: ping::Behaviour,
}


fn get_gossipsub_config() -> Result<Config, Box<dyn Error>> {
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;
    Ok(gossipsub_config)
}

fn publish_message(topic_name: String, message: &InputMessage, gossip_behaviour: &mut Behaviour) -> bool {
    let topic = gossipsub::IdentTopic::new(topic_name);
    let message_str = serde_json::to_string(&message).unwrap();

    let message_id = gossip_behaviour.publish(topic.clone(), message_str.as_bytes()).unwrap_or_else(|err| panic!("Failed to send message {:?}", err));
    println!("message sent {}", message_id);
    true
}