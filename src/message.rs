pub type ClientId = usize;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    NewClient(ClientId, mpsc::Sender<String>),
    Message(ClientId, String),
    Disconnect(ClientId),
}
