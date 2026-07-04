// Packet enum
pub enum Packet {
    Join(String),                              // Variation 1: Join
    Leave(String),                             // Variation 2: Leave
    Message { user: String, content: String }, // Variation 3: Message
}

pub struct Message {
    pub user: String,
    pub content: String,
}
