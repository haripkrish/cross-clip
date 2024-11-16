use std::{thread::sleep, time::Duration};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{RwLock};

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use flutter_rust_bridge::frb;
use futures::StreamExt;
use libp2p::{gossipsub, mdns, PeerId, Swarm};
use libp2p::{SwarmBuilder};
use libp2p::gossipsub::{Behaviour, Config};
use libp2p::identity::Keypair;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
// use log::{debug, error, info};
use tokio::{io, select};

use crate::frb_generated::StreamSink;
use crate::schema::message::InputMessage;
use crate::service::swarm_util::{get_gossipsub_config, initialize_swarm};
use crate::schema::swarm_model::{MyBehaviourEvent, MyBehaviour};

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

fn event_handler() {}
#[tokio::main]
pub async fn run_app(_s: StreamSink<CustomResponseEvent>) {
    println!("Starting Application RUN_APP");

    let (mut swarm, topic_id) = initialize_swarm(&"color cigar trouble domain floor math card festival hammer safe govern cute strong common patient");

    loop {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                    let line = "THIS IS A TEST MESSAGE";
                    let _message = InputMessage::new(line.to_string());
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
                _ => {
                }
            }
        }
    }
}







