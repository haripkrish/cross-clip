use flutter_rust_bridge::for_generated::StreamSinkBase;
use flutter_rust_bridge::frb;
use futures::StreamExt;
use libp2p::{gossipsub, mdns, Swarm};
use libp2p::swarm::SwarmEvent;
use serde::{Deserialize, Serialize};
use tokio::{io, select};
use tokio::io::AsyncBufReadExt;


pub mod schema;
pub mod service;
pub mod utils;
use crate::schema::message::InputMessage;
use crate::service::swarm_util::{publish_message, initialize_swarm};
use crate::schema::swarm_model::{MyBehaviourEvent, MyBehaviour};
use crate::utils::common::handle_swarm_event;
use crate::utils::common::DEFAULT_MNEMONIC;


#[tokio::main]
// #[frb(ignore)]
pub async fn main() {
    println!("Application starting");
    let (mut swarm, topic) = initialize_swarm(&DEFAULT_MNEMONIC);
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
             event = swarm.select_next_some() => handle_swarm_event(event, &mut swarm)
        }
    }
}