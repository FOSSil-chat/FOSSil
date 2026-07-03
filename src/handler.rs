use crate::packet::{Message, Packet};

pub fn packet_handler(packet_type: Packet, packet_payload: Message) {
    match packet_type {
        Packet::Message(text) => {
            println!("Message: {}", text)
        }
        Packet::Join(name) => {
            println!("Joined: {}", name)
        }
        Packet::Leave(name) => {
            println!("Left: {}", name)
        }
    }
}