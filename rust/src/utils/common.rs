use std::string::ToString;
use libp2p::{gossipsub, mdns, Swarm};
use libp2p::swarm::SwarmEvent;
use crate::schema::message::InputMessage;
use crate::schema::swarm_model::{MyBehaviourEvent, MyBehaviour};

pub static DEFAULT_MNEMONIC: &str = "color cigar trouble domain floor math card festival hammer safe govern cute strong common patient";

pub fn handle_swarm_event(event: SwarmEvent<MyBehaviourEvent>,  swarm: &mut Swarm<MyBehaviour>) {
    match event {
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Local node is listening onNN {address}");
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discovered a new peer: {peer_id}");
                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discover peer has expired: {peer_id}");
                swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                                              propagation_source: peer_id,
                                                              message_id: id,
                                                              message
                                                          })) => {
            let message_struct: InputMessage = serde_json::from_slice(&message.data).unwrap();
            println!(
                "Got message: '{}' and timestamp: '{}'\n id: {id} \n peer: {peer_id}",
                &message_struct.message, &message_struct.timestamp
            );
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Subscribed {
                                                              peer_id, topic
                                                          })) => {
            println!("Peer {peer_id} subscribed to topic: {topic}");
        }
        _ => {}
    }
}