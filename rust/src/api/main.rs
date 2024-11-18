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
use crate::utils::common::{DEFAULT_MNEMONIC, handle_swarm_event};
use crate::frb_generated::StreamSink;
use crate::schema::message::InputMessage;
use crate::service::swarm_util::{get_gossipsub_config, initialize_swarm, subscribe_to_topic};
use crate::schema::swarm_model::{MyBehaviourEvent, MyBehaviour, CustomResponseEvent};

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

const ONE_SECOND: Duration = Duration::from_secs(2);
static SWARM_EVENT_STREAM: RwLock<Option<StreamSink<CustomResponseEvent>>> = RwLock::new(None);

fn event_handler() {}
#[tokio::main]
pub async fn start_app(_s: StreamSink<CustomResponseEvent>) -> Result<()> {
    println!("Starting Application start_app");

    let (mut swarm, topic_id) = initialize_swarm(&DEFAULT_MNEMONIC);

    loop {
        select! {
            event = swarm.select_next_some() => handle_swarm_event(event, &mut swarm)
        }
    }
}








