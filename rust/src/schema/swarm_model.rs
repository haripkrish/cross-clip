use std::collections::HashMap;
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
