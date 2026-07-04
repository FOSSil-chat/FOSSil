// Imports & Declarations
mod handler;
mod packet;
mod server;
use crate::server::ServerState;
use handler::packet_handler;
use packet::Packet;

// (Testing) Prints the server state
fn print_state(state: &ServerState) {
    println!("\nConnected users: ");
    for user in &state.connected_users {
        println!("  - {}", user);
    }
    println!("\nMessages:");
    for message in &state.messages {
        println!("  - {} said '{}'", message.user, message.content);
    }
}

// Main function
fn main() {
    let mut state = ServerState {
        connected_users: Vec::new(), // Creates ServerState instance to track server state (e.g. connected users)
        messages: Vec::new(),
    };

    let packet1 = Packet::Join(String::from("Simarpreet-Singh")); // Creates and sends 3 fake packets to test packet handling functionality
    let packet2 = Packet::Message {
        user: "Simarpreet-Singh".to_string(),
        content: "Hello! This is a message to test the message functionality of the FOSSil Chat EDS 'Skeleton' Program (ironic, right?)".to_string(),
    };
    let packet3 = Packet::Leave(String::from("Simarpreet-Singh"));

    print_state(&state); // Prints server state
    packet_handler(&mut state, packet1); // Sends fake packets
    print_state(&state); 
    packet_handler(&mut state, packet2);
    print_state(&state); 
    packet_handler(&mut state, packet3);
    print_state(&state); 
}