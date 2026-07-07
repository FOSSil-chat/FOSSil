use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Packet {
    // Creates Packet enum
    Join(String),
    Leave(String),
    Message { user: String, content: String },
}
