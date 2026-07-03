// Imports & Declarations
mod handler;
mod packet;
mod server;
use crate::server::ServerState;
use handler::packet_handler;
use packet::Packet;

// Main function
fn main() {
    let mut state = ServerState {
        connected_users: Vec::new(),
    };
    let packet1 = Packet::Join(String::from("Simarpreet-Singh"));
    let packet2 = Packet::Message {
        user: "Simarpreet-Singh".to_string(),
        content: "Hello! This is a message to test the message functionality of the FOSSil Chat EDS 'Skeleton' Program (ironic, right?)".to_string(),
    };
    let packet3 = Packet::Leave(String::from("Simarpreet-Singh"));
    println!("\nConnected users: ");
    for user in &state.connected_users {
        println!("  - {}", user);
    }
    packet_handler(&mut state, packet1);
    println!("\nConnected users: ");
    for user in &state.connected_users {
        println!("  - {}", user);
    }
    packet_handler(&mut state, packet2);
    println!("\nConnected users: ");
    for user in &state.connected_users {
        println!("  - {}", user);
    }
    packet_handler(&mut state, packet3);
    println!("\nConnected users: ");
    for user in &state.connected_users {
        println!("  - {}", user);
    }
}
