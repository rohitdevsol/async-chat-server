use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::message::{ClientId, Event};

pub async fn run_server(mut rx: mpsc::Receiver<Event>) {
    let mut clients: HashMap<ClientId, mpsc::Sender<String>> = HashMap::new();

    while let Some(event) = rx.recv().await {
        match event {
            Event::NewClient(id, client_tx) => {
                clients.insert(id, client_tx);
                println!("Client {} connected", id);
            }

            Event::Message(id, msg) => {
                println!("Client {}: {}", id, msg);

                for (other_id, tx) in clients.iter() {
                    if *other_id != id {
                        let _ = tx.send(msg.clone()).await;
                    }
                }
            }

            Event::Disconnect(id) => {
                clients.remove(&id);
                println!("Client {} disconnected", id);
            }
        }
    }
}
