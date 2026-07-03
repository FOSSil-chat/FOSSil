use crate::packet::Packet;
use crate::server::ServerState;

pub fn packet_handler(state: &mut ServerState, packet_type: Packet) {
    match packet_type {
        Packet::Message { user, content } => {
            handle_message(user, content);
        }
        Packet::Join(name) => {
            handle_join(state, name);
        }
        Packet::Leave(name) => {
            handle_leave(state, name);
        }
    }
}

fn handle_join(state: &mut ServerState, name: String) {
    println!("\n{} joined.", name);
    state.connected_users.push(name);
}

fn handle_leave(state: &mut ServerState, name: String) {
    println!("\n{} left.", name);
    state.connected_users.retain(|user| user != &name);
}

fn handle_message(user: String, content: String) {
    println!("\n{} said '{}'", user, content)
}
