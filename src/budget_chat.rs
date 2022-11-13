use std::sync::{Arc, Mutex};
use std::thread;
use std::{collections::HashMap, sync::mpsc};

#[derive(Clone)]
pub struct Room {
    senders: Arc<Mutex<HashMap<String, mpsc::Sender<Message>>>>,
}

// TODO: this could use a single thread
impl Room {
    pub fn new() -> Room {
        Room {
            senders: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // TODO: handle duplicate names
    pub fn join(&mut self, name: &str) -> (mpsc::Sender<Message>, mpsc::Receiver<Message>) {
        let (public_sender, public_receiver) = mpsc::channel::<Message>();
        self.senders
            .lock()
            .unwrap()
            .insert(name.to_string(), public_sender);

        let (private_sender, private_receiver) = mpsc::channel::<Message>();

        let name = name.to_string();
        let senders = Arc::clone(&self.senders);
        thread::spawn(move || {
            'outer: while let Ok(msg) = private_receiver.recv() {
                for (k, v) in senders.lock().unwrap().iter() {
                    if *k == name {
                        continue;
                    }
                    if let Err(_) = v.send(msg.clone()) {
                        break 'outer;
                    }
                }
            }
            senders.lock().unwrap().remove(&name);
        });

        (private_sender, public_receiver)
    }

    pub fn users(&self) -> Vec<String> {
        return self
            .senders
            .lock()
            .unwrap()
            .keys()
            .map(|k| k.to_string())
            .collect();
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Join { user: String },
    Leave { user: String },
    Chat { user: String, content: String },
}

pub fn is_legal_name(name: &str) -> bool {
    name.chars().all(|ch| ch.is_alphanumeric())
}
