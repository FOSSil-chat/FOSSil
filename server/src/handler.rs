// Imports and declarations
use crate::server::{Client, ServerState};
use crate::tcp::send_error;
use chrono::DateTime;
use chrono::Utc;
use fossil_shared::message::Message;
use fossil_shared::packet::Packet;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn packet_handler(
    state: Arc<Mutex<ServerState>>,
    packet_type: Packet,
    sender: tokio::sync::mpsc::UnboundedSender<Packet>,
) {
    // Packet handler function
    match packet_type {
        Packet::Message { user, content } => {
            let user_clone = user.clone();
            let content_clone = content.clone();

            // If the packet type is 'message' it calls the handle_message() function
            match handle_message(state, user, content).await {
                Ok((id, timestamp)) => println!(
                    "{} said '{}' (ID {}) at {}.",
                    user_clone,
                    content_clone,
                    id,
                    DateTime::<Utc>::from_timestamp_millis(timestamp).unwrap()
                ),
                Err(e) => {
                    let _ = send_error(&sender, "ERROR_MESSAGE_FAILED".to_string()).await;
                    println!("Error sending message: '{}'", e);
                }
            }
        }
        Packet::Join(name) => {
            // If the packet type is 'Join', it calls the handle_join() function
            match handle_join(state.clone(), name, sender).await {
                Ok(_) => {}
                Err(e) => println!("User joining failed: '{}'", e),
            }
        }
        Packet::Leave(name) => {
            // If the packet type is 'Leave', it calls the handle_leave() function
            match handle_leave(state.clone(), name).await {
                Ok(_) => {}
                Err(e) => println!("User leaving failed: '{}'", e),
            }
        }
        Packet::Error(error_type) => {
            // If the packet type is 'Error', it prints the error type
            println!("Received error: {}", error_type);
        }
    }
}

pub async fn handle_join(
    state: Arc<Mutex<ServerState>>,
    name: String,
    sender: tokio::sync::mpsc::UnboundedSender<Packet>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    if name.is_empty() {
        return Err("Error: Name cannot be empty.".to_string());
    }
    if state.clients.iter().any(|c| c.name == name) {
        let _ = send_error(&sender, "ERROR_USER_EXISTS".to_string()).await;
        return Err("Error: User already joined.".to_string());
    }
    // Join handler
    println!("\n{} joined.", name); // Prints that a user joined
    state.clients.push(Client { name, sender }); // Adds the client to the ServerState's clients Vec
    Ok(())
}

pub async fn handle_leave(state: Arc<Mutex<ServerState>>, name: String) -> Result<(), String> {
    let mut state = state.lock().await;
    if name.is_empty() {
        return Err("Error: Name cannot be empty.".to_string());
    }
    if !state.clients.iter().any(|c| c.name == name) {
        return Err("Error: User does not exist.".to_string());
    }
    // Leave handler
    println!("\n{} left.", name); // Print that a user left
    state.clients.retain(|client| client.name != name); // Removes the client from the ServerState's clients Vec
    Ok(())
}

pub async fn handle_message(
    state: Arc<Mutex<ServerState>>,
    user: String,
    content: String,
) -> Result<(u64, i64), String> {
    let mut state = state.lock().await;
    if user.is_empty() {
        return Err("Error: Message does not have a sender.".to_string()); // Enforces sender
    }
    if content.is_empty() {
        return Err("Error: Message does not have content.".to_string()); // Enforces content
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

    // Broadcast message to all connected clients
    let message_packet = Packet::Message { user, content };
    for client in &state.clients {
        let _ = client.sender.send(message_packet.clone());
    }

    Ok((id, timestamp))
}
