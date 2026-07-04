// Imports and declarations
use crate::packet::Message;
use crate::packet::Packet;
use crate::server::ServerState;
use chrono::Utc;

pub fn packet_handler(state: &mut ServerState, packet_type: Packet) {
    // Packet handler function
    match packet_type {
        Packet::Message { user, content } => {
            // If the packet type is 'message' it calls the handle_message() function
            handle_message(state, user, content);
        }
        Packet::Join(name) => {
            // If the packet type is 'Join', it calls the handle_join() function
            handle_join(state, name);
        }
        Packet::Leave(name) => {
            // If the packet type is 'Leave', it calls the handle_leave() function
            handle_leave(state, name);
        }
    }
}

fn handle_join(state: &mut ServerState, name: String) {
    // Join handler
    println!("\n{} joined.", name); // Prints that a user joined
    state.connected_users.push(name); // Adds the user to the ServerState's connected_users Vec
}

fn handle_leave(state: &mut ServerState, name: String) {
    // Leave handler
    println!("\n{} left.", name); // Print that a user left
    state.connected_users.retain(|user| user != &name); // Removes the user from the ServerState's connected_users Vec (retain() requires a list of all items to keep, so this inline function outputs all users except the one to remove)
}

fn handle_message(state: &mut ServerState, user: String, content: String) {
    let timestamp: u64 = Utc::now().timestamp_millis() as u64;
    println!("\n{} said '{}'", user, content); // Prints that the user sent a message, with the message content and sender
    let id = state.next_message_id; // Takes the message ID as a variable
    state.messages.push(Message {
        // Pushes a Message to the messages Vec
        id,
        user,
        content,
        timestamp,
    });
    state.next_message_id += 1; // Increments the next message ID counter
}
