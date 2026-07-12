use std::sync::Arc;

use fossil_server::handler::{handle_join, handle_leave, handle_message, packet_handler};
use fossil_server::server::ServerState;
use fossil_shared::packet::Packet;
use tokio::sync::Mutex;

fn create_state() -> Arc<Mutex<ServerState>> {
    Arc::new(Mutex::new(ServerState {
        clients: Vec::new(),
        messages: Vec::new(),
        next_message_id: 0,
    }))
}

fn create_client_channel() -> (tokio::sync::mpsc::UnboundedSender<Packet>, tokio::sync::mpsc::UnboundedReceiver<Packet>) {
    tokio::sync::mpsc::unbounded_channel()
}

#[tokio::test]
async fn test_user_can_join() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    let result = handle_join(state.clone(), "Alice".to_string(), tx).await;

    assert!(result.is_ok());

    let state = state.lock().await;

    assert_eq!(state.clients.len(), 1);
    assert_eq!(state.clients[0].name, "Alice");
}

#[tokio::test]
async fn test_empty_username_fails() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    let result = handle_join(state, "".to_string(), tx).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_duplicate_user_fails() {
    let state = create_state();
    let (tx1, _rx1) = create_client_channel();
    let (tx2, _rx2) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx1)
        .await
        .unwrap();

    let result = handle_join(state, "Alice".to_string(), tx2).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_can_leave() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    let result = handle_leave(state.clone(), "Alice".to_string()).await;

    assert!(result.is_ok());

    let state = state.lock().await;
    assert_eq!(state.clients.len(), 0);
}

#[tokio::test]
async fn test_leaving_unknown_user_fails() {
    let state = create_state();

    let result = handle_leave(state, "Bob".to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_message_creation() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    let result = handle_message(state.clone(), "Alice".to_string(), "Hello".to_string()).await;

    assert!(result.is_ok());

    let state = state.lock().await;

    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].user, "Alice");
    assert_eq!(state.messages[0].content, "Hello");
}

#[tokio::test]
async fn test_empty_message_fails() {
    let state = create_state();

    let result = handle_message(state, "Alice".to_string(), "".to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_empty_sender_fails() {
    let state = create_state();

    let result = handle_message(state, "".to_string(), "Hello".to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_message_ids_increment() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    handle_message(state.clone(), "Alice".to_string(), "One".to_string())
        .await
        .unwrap();

    handle_message(state.clone(), "Alice".to_string(), "Two".to_string())
        .await
        .unwrap();

    let state = state.lock().await;

    assert_eq!(state.messages[0].id, 0);
    assert_eq!(state.messages[1].id, 1);
}

#[tokio::test]
async fn test_packet_handler_message() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx.clone())
        .await
        .unwrap();

    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "Hello".to_string(),
    };

    packet_handler(state.clone(), packet, tx).await;

    let state = state.lock().await;
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].user, "Alice");
}

#[tokio::test]
async fn test_broadcast_message_to_single_client() {
    let state = create_state();
    let (tx, mut rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    handle_message(
        state.clone(),
        "Alice".to_string(),
        "Hello".to_string(),
    )
    .await
    .unwrap();

    // Client should receive the message
    let received = rx.recv().await;
    assert!(received.is_some());
    
    if let Some(Packet::Message { user, content }) = received {
        assert_eq!(user, "Alice");
        assert_eq!(content, "Hello");
    } else {
        panic!("Expected Message packet");
    }
}

#[tokio::test]
async fn test_broadcast_message_to_multiple_clients() {
    let state = create_state();
    let (tx_alice, mut rx_alice) = create_client_channel();
    let (tx_bob, mut rx_bob) = create_client_channel();
    let (tx_charlie, mut rx_charlie) = create_client_channel();

    // All clients join
    handle_join(state.clone(), "Alice".to_string(), tx_alice)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), tx_bob)
        .await
        .unwrap();
    handle_join(state.clone(), "Charlie".to_string(), tx_charlie)
        .await
        .unwrap();

    // Alice sends a message
    handle_message(
        state.clone(),
        "Alice".to_string(),
        "Hello everyone!".to_string(),
    )
    .await
    .unwrap();

    // All clients should receive the message
    let alice_received = rx_alice.recv().await;
    let bob_received = rx_bob.recv().await;
    let charlie_received = rx_charlie.recv().await;

    assert!(alice_received.is_some());
    assert!(bob_received.is_some());
    assert!(charlie_received.is_some());

    // Verify message content for each
    if let Some(Packet::Message { user, content }) = alice_received {
        assert_eq!(user, "Alice");
        assert_eq!(content, "Hello everyone!");
    } else {
        panic!("Expected Message packet for Alice");
    }

    if let Some(Packet::Message { user, content }) = bob_received {
        assert_eq!(user, "Alice");
        assert_eq!(content, "Hello everyone!");
    } else {
        panic!("Expected Message packet for Bob");
    }

    if let Some(Packet::Message { user, content }) = charlie_received {
        assert_eq!(user, "Alice");
        assert_eq!(content, "Hello everyone!");
    } else {
        panic!("Expected Message packet for Charlie");
    }
}

#[tokio::test]
async fn test_multiple_messages_broadcast_to_all() {
    let state = create_state();
    let (tx_alice, mut rx_alice) = create_client_channel();
    let (tx_bob, mut rx_bob) = create_client_channel();

    // Clients join
    handle_join(state.clone(), "Alice".to_string(), tx_alice)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), tx_bob)
        .await
        .unwrap();

    // Alice sends first message
    handle_message(
        state.clone(),
        "Alice".to_string(),
        "First message".to_string(),
    )
    .await
    .unwrap();

    // Bob sends second message
    handle_message(
        state.clone(),
        "Bob".to_string(),
        "Second message".to_string(),
    )
    .await
    .unwrap();

    // Alice should receive both messages
    let msg1 = rx_alice.recv().await;
    let msg2 = rx_alice.recv().await;

    assert!(msg1.is_some());
    assert!(msg2.is_some());

    // Bob should receive both messages
    let msg1_bob = rx_bob.recv().await;
    let msg2_bob = rx_bob.recv().await;

    assert!(msg1_bob.is_some());
    assert!(msg2_bob.is_some());

    // Verify message order
    if let Some(Packet::Message { user: u1, content: c1 }) = msg1 {
        assert_eq!(u1, "Alice");
        assert_eq!(c1, "First message");
    }

    if let Some(Packet::Message { user: u2, content: c2 }) = msg2 {
        assert_eq!(u2, "Bob");
        assert_eq!(c2, "Second message");
    }
}

#[tokio::test]
async fn test_client_disconnection_removes_from_clients() {
    let state = create_state();
    let (tx_alice, _rx_alice) = create_client_channel();
    let (tx_bob, _rx_bob) = create_client_channel();

    // Both clients join
    handle_join(state.clone(), "Alice".to_string(), tx_alice)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), tx_bob)
        .await
        .unwrap();

    let state_locked = state.lock().await;
    assert_eq!(state_locked.clients.len(), 2);
    drop(state_locked);

    // Alice leaves
    handle_leave(state.clone(), "Alice".to_string())
        .await
        .unwrap();

    let state_locked = state.lock().await;
    assert_eq!(state_locked.clients.len(), 1);
    assert_eq!(state_locked.clients[0].name, "Bob");
}

#[tokio::test]
async fn test_all_clients_stored_in_vec() {
    let state = create_state();

    // Join 5 clients
    for i in 1..=5 {
        let name = format!("User{}", i);
        let (tx, _rx) = create_client_channel();
        handle_join(state.clone(), name, tx).await.unwrap();
    }

    let state_locked = state.lock().await;
    assert_eq!(state_locked.clients.len(), 5);

    // Verify all names are stored
    assert!(state_locked.clients.iter().any(|c| c.name == "User1"));
    assert!(state_locked.clients.iter().any(|c| c.name == "User2"));
    assert!(state_locked.clients.iter().any(|c| c.name == "User3"));
    assert!(state_locked.clients.iter().any(|c| c.name == "User4"));
    assert!(state_locked.clients.iter().any(|c| c.name == "User5"));
}

#[tokio::test]
async fn test_message_persisted_in_messages_vec() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    // Send multiple messages
    handle_message(state.clone(), "Alice".to_string(), "Message 1".to_string())
        .await
        .unwrap();
    handle_message(state.clone(), "Alice".to_string(), "Message 2".to_string())
        .await
        .unwrap();
    handle_message(state.clone(), "Alice".to_string(), "Message 3".to_string())
        .await
        .unwrap();

    let state_locked = state.lock().await;
    assert_eq!(state_locked.messages.len(), 3);
    assert_eq!(state_locked.messages[0].content, "Message 1");
    assert_eq!(state_locked.messages[1].content, "Message 2");
    assert_eq!(state_locked.messages[2].content, "Message 3");
}

#[tokio::test]
async fn test_packet_handler_join() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();
    let packet = Packet::Join("Bob".to_string());

    packet_handler(state.clone(), packet, tx).await;

    let state = state.lock().await;
    assert!(state.clients.iter().any(|c| c.name == "Bob"));
}

#[tokio::test]
async fn test_packet_handler_leave() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Bob".to_string(), tx.clone())
        .await
        .unwrap();

    let packet = Packet::Leave("Bob".to_string());
    packet_handler(state.clone(), packet, tx).await;

    let state = state.lock().await;
    assert!(!state.clients.iter().any(|c| c.name == "Bob"));
}

#[tokio::test]
async fn test_multiple_users_join() {
    let state = create_state();

    let (tx1, _rx1) = create_client_channel();
    let (tx2, _rx2) = create_client_channel();
    let (tx3, _rx3) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx1)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), tx2)
        .await
        .unwrap();
    handle_join(state.clone(), "Charlie".to_string(), tx3)
        .await
        .unwrap();

    let state = state.lock().await;

    assert_eq!(state.clients.len(), 3);
    assert!(state.clients.iter().any(|c| c.name == "Alice"));
    assert!(state.clients.iter().any(|c| c.name == "Bob"));
    assert!(state.clients.iter().any(|c| c.name == "Charlie"));
}

#[tokio::test]
async fn test_message_timestamp_is_valid() {
    let state = create_state();
    let (tx, _rx) = create_client_channel();

    handle_join(state.clone(), "Alice".to_string(), tx)
        .await
        .unwrap();

    let (_, timestamp) = handle_message(
        state.clone(),
        "Alice".to_string(),
        "Hello".to_string(),
    )
    .await
    .unwrap();
    assert!(timestamp > 0);

    let state = state.lock().await;
    assert_eq!(state.messages[0].timestamp, timestamp);
}

#[tokio::test]
async fn test_broadcast_to_only_existing_clients() {
    let state = create_state();
    let (tx_alice, mut rx_alice) = create_client_channel();
    let (tx_bob, mut rx_bob) = create_client_channel();
    let (tx_dropped, _rx_dropped) = create_client_channel();

    // Alice, Bob, and a third user join
    handle_join(state.clone(), "Alice".to_string(), tx_alice)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), tx_bob)
        .await
        .unwrap();
    handle_join(state.clone(), "Dropped".to_string(), tx_dropped)
        .await
        .unwrap();

    // Remove the third user
    handle_leave(state.clone(), "Dropped".to_string())
        .await
        .unwrap();

    // Alice sends a message
    handle_message(
        state.clone(),
        "Alice".to_string(),
        "Message".to_string(),
    )
    .await
    .unwrap();

    // Only Alice and Bob should receive it
    assert!(rx_alice.recv().await.is_some());
    assert!(rx_bob.recv().await.is_some());
}

