// Imports and declarations
use crate::packet::Message;
use crate::packet::Packet;
use crate::server::ServerState;
use chrono::DateTime;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use crate::tcp::send_error;

pub fn packet_handler(state: Arc<Mutex<ServerState>>, packet_type: Packet) {
    // Packet handler function
    match packet_type {
        Packet::Message { user, content } => {
            let user_clone = user.clone();
            let content_clone = content.clone();

            // If the packet type is 'message' it calls the handle_message() function
            match handle_message(state, user, content) {
                Ok((id, timestamp)) => println!(
                    "{} said '{}' (ID {}) at {}.",
                    user_clone,
                    content_clone,
                    id,
                    DateTime::<Utc>::from_timestamp_millis(timestamp).unwrap()
                ),
                Err(e) => println!("Error sending message: '{}'", e),
            }
        }
        Packet::Join(name) => {
            // If the packet type is 'Join', it calls the handle_join() function
            match handle_join(state, name) {
                Ok(_) => {}
                Err(e) => println!("User joining failed: '{}'", e),
            }
        }
        Packet::Leave(name) => {
            // If the packet type is 'Leave', it calls the handle_leave() function
            match handle_leave(state, name) {
                Ok(_) => {}
                Err(e) => println!("User leaving failed: '{}'", e),
            }
        }
    }
}

fn handle_join(state: Arc<Mutex<ServerState>>, name: String) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if name.is_empty() {
        return Err("Error: Name cannot be empty.".to_string());
    }
    if state.connected_users.contains(&name) {
        send_error("ERROR_USER_EXISTS".to_string());
        return Err("Error: User already joined.".to_string());
    }
    // Join handler
    println!("\n{} joined.", name); // Prints that a user joined
    state.connected_users.push(name); // Adds the user to the ServerState's connected_users Vec
    Ok(())
}

fn handle_leave(state: Arc<Mutex<ServerState>>, name: String) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if name.is_empty() {
        send_error("ERROR_NAME_EMPTY".to_string());
        return Err("Error: Name cannot be empty.".to_string());
    }
    if !state.connected_users.contains(&name) {
        send_error("ERROR_USER_NOT_EXISTS".to_string());
        return Err("Error: User does not exist.".to_string());
    }
    // Leave handler
    println!("\n{} left.", name); // Print that a user left
    state.connected_users.retain(|user| user != &name); // Removes the user from the ServerState's connected_users Vec (retain() requires a list of all items to keep, so this inline function outputs all users except the one to remove)
    Ok(())
}

fn handle_message(
    state: Arc<Mutex<ServerState>>,
    user: String,
    content: String,
) -> Result<(u64, i64), String> {
    let mut state = state.lock().unwrap();
    if user.is_empty() {
        send_error("ERROR_MISSING_SENDER".to_string());
        return Err("Error: Message does not have a sender.".to_string());
    }
    if content.is_empty() {
        send_error("ERROR_MISSING_CONTENT".to_string());
        return Err("Error: Message does not have content.".to_string());
    }
    let timestamp = Utc::now().timestamp_millis();
    let id = state.next_message_id;
    state.messages.push(Message {
        // Pushes a Message to the messages Vec
        id,
        user: user.clone(),
        content: content.clone(),
        timestamp,
    });
    state.next_message_id += 1; // Increments the next message ID counter
    Ok((id, timestamp))
}
