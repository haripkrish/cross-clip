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

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NoteStorage {
    pub data: HashMap<String, Vec<HashMap<String, String>>>
}

impl NoteStorage {
    fn new() -> Self {
        NoteStorage {
            data: HashMap::new()
        }
    }

    fn set_note(&mut self, user_device_id: &str, note_header: &str, note_value: &str) {
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

    let mut note_storage = NoteStorage::new();

    println!("Get note by user device ID");
    if let Some(notes ) = note_storage.get_note("asd") {
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

    if let Some(notes ) = note_storage.get_note(user_device_id_test) {
        println!("Note for {}", user_device_id_test);
        for note in notes {
            for (k, v) in note {
                println!("Note header: {}, Notes: {}", k, v)
            }

        }
        println!("\n");
    }

    println!("Note storage: {:?}",note_storage);
    format!("test")
}


#[flutter_rust_bridge::frb(sync)]
pub fn test_4() -> String {
    println!("Test note storage");

    let mut note_storage = NoteStorage::new();
    println!("Set note by user device ID");
    note_storage.set_note("Hari_device1", "Note_header1", "Note_value1");
    println!("Get note by user device ID");
    let user_device_id_test = "Hari_device1";

    if let Some(notes ) = note_storage.get_note(user_device_id_test) {
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


use anyhow::Result;
use std::{thread::sleep, time::Duration};

use crate::frb_generated::StreamSink;

const ONE_SECOND: Duration = Duration::from_secs(2);
//static NOTE_STREAM: RwLock<Option<StreamSink<NoteStorage>>> = RwLock::new(None);

// can't omit the return type yet, this is a bug
//#[flutter_rust_bridge::frb(sync)]
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

use tokio::{io, select};
use libp2p::identity::Keypair;
use libp2p::PeerId;

use std::sync::{RwLock, TryLockResult};

static NOTES_EVENT_STREAM: RwLock<Option<StreamSink<NoteStorage>>> = RwLock::new(None);

//pub fn note_event_stream(sink: SteamSink

pub fn notes_event_stream(s: StreamSink<NoteStorage>) -> Result<()> {

    println!("Note event called");
    let mut note_storage = NoteStorage::new();
    let mut ticks = 0;
    loop {
        s.add(note_storage.clone());
        sleep(ONE_SECOND);
        if 100 == i32::MAX {
            break;
        }
        println!("Set note by user device ID");
        note_storage.set_note("Hari_device1", "Note_header1", &format!("{}{}", "", ticks));
        ticks += 1;
    }
    Ok(())
}