// Imports & Declarations
mod handler;
mod packet;
mod server;
mod tcp;

use crate::server::ServerState;
use std::sync::{Arc, Mutex};
use tcp::tcp_listener;

// Main function
fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        connected_users: Vec::new(), // Creates ServerState instance to track server state (e.g. connected users)
        messages: Vec::new(),
        next_message_id: 0,
    }));

    tcp_listener(state); // Calls tcp_listener() function in tcp.rs
}
