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
            match handle_message(state, user, content) {
                Ok(_) => println!(""),
                Err(e) => println!("Error sending message: '{}'", e),
            }
        }
        Packet::Join(name) => {
            // If the packet type is 'Join', it calls the handle_join() function
            match handle_join(state, name) {
                Ok(_) => println!(""),
                Err(e) => println!("User joining failed: '{}'", e),
            }
        }
        Packet::Leave(name) => {
            // If the packet type is 'Leave', it calls the handle_leave() function
            match handle_leave(state, name) {
                Ok(_) => println!(""),
                Err(e) => println!("User leaving failed: '{}'", e),
            }
        }
    }
}

fn handle_join(state: &mut ServerState, name: String) -> Result<(), String> {
    if name.is_empty() {
        return Err("Error: Name cannot be empty.".to_string());
    }
    if state.connected_users.contains(&name) {
        return Err("Error: User already joined.".to_string());
    }
    // Join handler
    println!("\n{} joined.", name); // Prints that a user joined
    state.connected_users.push(name); // Adds the user to the ServerState's connected_users Vec
    Ok(())
}

fn handle_leave(state: &mut ServerState, name: String) -> Result<(), String> {
    if name.is_empty() {
        return Err("Error: Name cannot be empty.".to_string());
    }
    if !state.connected_users.contains(&name) {
        return Err("Error: User does not exist.".to_string());
    }
    // Leave handler
    println!("\n{} left.", name); // Print that a user left
    state.connected_users.retain(|user| user != &name); // Removes the user from the ServerState's connected_users Vec (retain() requires a list of all items to keep, so this inline function outputs all users except the one to remove)
    Ok(())
}

fn handle_message(state: &mut ServerState, user: String, content: String) -> Result<(), String> {
    if user.is_empty() {
        return Err("Error: Message does not have a sender.".to_string());
    }
    if content.is_empty() {
        return Err("Error: Message does not have content.".to_string());
    }
    let timestamp: i64 = Utc::now().timestamp_millis();
    println!("{} said '{}'", user, content); // Prints that the user sent a message, with the message content and sender
    let id = state.next_message_id; // Takes the message ID as a variable
    state.messages.push(Message {
        // Pushes a Message to the messages Vec
        id,
        user,
        content,
        timestamp,
    });
    state.next_message_id += 1; // Increments the next message ID counter
    Ok(())
}
