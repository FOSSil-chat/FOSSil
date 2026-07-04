// Imports and declarations
use crate::packet::Message;

pub struct ServerState {
    // ServerState struct
    pub connected_users: Vec<String>, // Connected users Vec
    pub messages: Vec<Message>,       // Messages Vec storing Message struct
    pub next_message_id: u64,         // Stores ID to use for the next message
}
