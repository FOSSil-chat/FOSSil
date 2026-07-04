// Imports & Declarations
mod handler;
mod packet;
mod server;
use crate::server::ServerState;
use chrono::{TimeZone, Utc};
use handler::packet_handler;
use packet::Packet;

// (Testing) Prints the server state
fn print_state(state: &ServerState) {
    println!("\nConnected users: ");
    for user in &state.connected_users {
        // Iterates over Vec, printing each username
        println!("  - {}", user);
    }
    println!("\nMessages:");
    for message in &state.messages {
        // Iterates over messages, printing each message
        let date_time = Utc.timestamp_millis_opt(message.timestamp as i64).unwrap();
        println!(
            "  - {} said '{}' (ID {}) at {}",
            message.user, message.content, message.id, date_time
        );
    }
}

// Main function
fn main() {
    let mut state = ServerState {
        connected_users: Vec::new(), // Creates ServerState instance to track server state (e.g. connected users)
        messages: Vec::new(),
        next_message_id: 0,
    };

    let packet1 = Packet::Join(String::from("Simarpreet-Singh")); // Creates and sends 3 fake packets to test packet handling functionality
    let packet2 = Packet::Message {
        user: "Simarpreet-Singh".to_string(),
        content:
            "Hello! This is a message to test the message functionality of FOSSil Chat - message 1!"
                .to_string(),
    };
    let packet3 = Packet::Message {
        user: "Simarpreet-Singh".to_string(),
        content:
            "Hello! This is a message to test the message functionality of FOSSil Chat - message 2!"
                .to_string(),
    };
    let packet4 = Packet::Message {
        user: "Simarpreet-Singh".to_string(),
        content:
            "Hello! This is a message to test the message functionality of FOSSil Chat - message 3!"
                .to_string(),
    };
    let packet5 = Packet::Leave(String::from("Simarpreet-Singh"));

    print_state(&state); // Prints server state
    packet_handler(&mut state, packet1); // Sends fake packets
    print_state(&state);
    packet_handler(&mut state, packet2);
    print_state(&state);
    packet_handler(&mut state, packet3);
    print_state(&state);
    packet_handler(&mut state, packet4);
    print_state(&state);
    packet_handler(&mut state, packet5);
    print_state(&state);
}
