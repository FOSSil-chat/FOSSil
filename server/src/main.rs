use std::sync::{Arc, Mutex};
use fossil_server::server::ServerState;
use fossil_server::tcp::tcp_listener;

fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        connected_users: Vec::new(),
        messages: Vec::new(),
        next_message_id: 0,
    }));

    tcp_listener(state);
}