// Imports and declarations
use fossil_shared::message::Message;
use fossil_shared::packet::Packet;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Client {
    pub name: String,
    pub sender: mpsc::UnboundedSender<Packet>,
}

pub struct ServerState {
    // ServerState struct
    pub clients: Vec<Client>,         // Connected clients with their message senders
    pub messages: Vec<Message>,       // Messages Vec storing Message struct
    pub next_message_id: u64,         // Stores ID to use for the next message
}
