use std::sync::{Arc, Mutex};

use fossil_server::handler::{handle_join, handle_leave, handle_message};
use fossil_server::server::ServerState;

fn create_state() -> Arc<Mutex<ServerState>> {
    Arc::new(Mutex::new(ServerState {
        connected_users: Vec::new(),
        messages: Vec::new(),
        next_message_id: 0,
    }))
}

#[test]
fn test_user_can_join() {
    let state = create_state();

    let result = handle_join(state.clone(), "Alice".to_string());

    assert!(result.is_ok());

    let state = state.lock().unwrap();

    assert_eq!(state.connected_users, vec!["Alice"]);
}

#[test]
fn test_empty_username_fails() {
    let state = create_state();

    let result = handle_join(state, "".to_string());

    assert!(result.is_err());
}

#[test]
fn test_duplicate_user_fails() {
    let state = create_state();

    handle_join(state.clone(), "Alice".to_string()).unwrap();

    let result = handle_join(state, "Alice".to_string());

    assert!(result.is_err());
}

#[test]
fn test_user_can_leave() {
    let state = create_state();

    handle_join(state.clone(), "Alice".to_string()).unwrap();

    let result = handle_leave(state.clone(), "Alice".to_string());

    assert!(result.is_ok());

    let state = state.lock().unwrap();

    assert!(state.connected_users.is_empty());
}

#[test]
fn test_leaving_unknown_user_fails() {
    let state = create_state();

    let result = handle_leave(state, "Bob".to_string());

    assert!(result.is_err());
}

#[test]
fn test_message_creation() {
    let state = create_state();

    let result = handle_message(state.clone(), "Alice".to_string(), "Hello".to_string());

    assert!(result.is_ok());

    let state = state.lock().unwrap();

    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].user, "Alice");
    assert_eq!(state.messages[0].content, "Hello");
}

#[test]
fn test_empty_message_fails() {
    let state = create_state();

    let result = handle_message(state, "Alice".to_string(), "".to_string());

    assert!(result.is_err());
}

#[test]
fn test_message_ids_increment() {
    let state = create_state();

    handle_message(state.clone(), "Alice".to_string(), "One".to_string()).unwrap();

    handle_message(state.clone(), "Alice".to_string(), "Two".to_string()).unwrap();

    let state = state.lock().unwrap();

    assert_eq!(state.messages[0].id, 0);
    assert_eq!(state.messages[1].id, 1);
}
