pub enum Packet {
    Join(String),
    Leave(String),
    Message { user: String, content: String },
}
