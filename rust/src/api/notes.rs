use std::collections::HashMap;

#[derive(Debug)]
struct NoteStorage {
    data: HashMap<String, Vec<HashMap<String, String>>>
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

#[flutter_rust_bridge::frb(sync)]
fn main() {
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
}