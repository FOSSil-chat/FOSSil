use serde::Deserialize;
use serde::Serialize;

// Packet enum
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Packet {
    Join(String),                              // Variation 1: Join
    Leave(String),                             // Variation 2: Leave
    Message { user: String, content: String }, // Variation 3: Message
    Error(String),                             // Variation 4: Error
}
