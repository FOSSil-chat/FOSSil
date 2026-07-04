// Packet enum
pub enum Packet {
    Join(String),                              // Variation 1: Join
    Leave(String),                             // Variation 2: Leave
    Message { user: String, content: String }, // Variation 3: Message
}

pub struct Message {
    pub id: u64,         // Message ID
    pub user: String,    // Name of the user who sent the message
    pub content: String, // Content of the message
    pub timestamp: u64,  // Timestamp of the message
}
