use flutter_rust_bridge::for_generated::StreamSinkBase;
use flutter_rust_bridge::frb;
use futures::StreamExt;
use libp2p::{gossipsub, mdns};
use libp2p::swarm::SwarmEvent;
use serde::{Deserialize, Serialize};
use tokio::{io, select};
use tokio::io::AsyncBufReadExt;

pub mod schema;
pub mod service;
use crate::schema::message::InputMessage;
use crate::service::swarm_util::{publish_message, initialize_swarm, subscribe_to_topic};
use crate::schema::swarm_model::{MyBehaviourEvent, MyBehaviour};


#[tokio::main]
#[frb(ignore)]
pub async fn main() {
    println!("Application starting");
    let (mut swarm, topic) = initialize_swarm(&"color cigar trouble domain floor math card festival hammer safe govern cute strong common patient");
    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    loop {
        select! {
              Ok(Some(line)) = stdin.next_line() => {
                         println!("{}",line);
                         let message = InputMessage::new(line);
                         println!("{:?}", message);
                         publish_message(&topic, &message, &mut swarm.behaviour_mut().gossipsub);
             }
             event = swarm.select_next_some() => match event {
                     SwarmEvent::NewListenAddr { address, .. } => {
                         println!("Local node is listening onNN {address}");
                         subscribe_to_topic(&topic, &mut swarm);
                         println!("THIS IS A TEST MESSAGE");
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
    }
}