use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Packet {
    Join(String),
    Leave(String),
    Message {
        user: String,
        content: String,
    },
}