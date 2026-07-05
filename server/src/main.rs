// Imports & Declarations
mod handler;
mod packet;
mod server;
mod tcp;
use crate::server::ServerState;
use chrono::{TimeZone, Utc};
use handler::packet_handler;
use packet::Packet;
use std::net::TcpListener;
use tcp::tcp_listener;

// Main function
fn main() {
    let mut state = ServerState {
        connected_users: Vec::new(), // Creates ServerState instance to track server state (e.g. connected users)
        messages: Vec::new(),
        next_message_id: 0,
    };

    tcp_listener(&mut state);
}
