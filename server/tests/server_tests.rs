use std::sync::Arc;

use fossil_server::handler::{handle_join, handle_leave, handle_message, packet_handler};
use fossil_server::server::ServerState;
use fossil_server::tcp::send_error;
use fossil_shared::packet::Packet;
use tokio::sync::Mutex;

fn create_state() -> Arc<Mutex<ServerState>> {
    Arc::new(Mutex::new(ServerState {
        connected_users: Vec::new(),
        messages: Vec::new(),
        next_message_id: 0,
    }))
}

// Mock writer for testing async functions
#[allow(dead_code)]
struct MockWriter {
    data: Vec<u8>,
    flush_count: usize,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter {
            data: Vec::new(),
            flush_count: 0,
        }
    }
}

impl tokio::io::AsyncWrite for MockWriter {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        self.data.extend_from_slice(buf);
        std::task::Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.flush_count += 1;
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
}

#[tokio::test]
async fn test_user_can_join() {
    let state = create_state();
    let mut writer = MockWriter::new();

    let result = handle_join(state.clone(), "Alice".to_string(), &mut writer).await;

    assert!(result.is_ok());

    let state = state.lock().await;

    assert_eq!(state.connected_users, vec!["Alice"]);
}

#[tokio::test]
async fn test_empty_username_fails() {
    let state = create_state();
    let mut writer = MockWriter::new();

    let result = handle_join(state, "".to_string(), &mut writer).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_duplicate_user_fails() {
    let state = create_state();
    let mut writer = MockWriter::new();

    handle_join(state.clone(), "Alice".to_string(), &mut writer)
        .await
        .unwrap();

    let result = handle_join(state, "Alice".to_string(), &mut writer).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_can_leave() {
    let state = create_state();
    let mut writer = MockWriter::new();

    handle_join(state.clone(), "Alice".to_string(), &mut writer)
        .await
        .unwrap();
    let mut writer = MockWriter::new();

    let result = handle_leave(state.clone(), "Alice".to_string(), &mut writer).await;

    println!("leave result: {:?}", result);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_leaving_unknown_user_fails() {
    let state = create_state();
    let mut writer = MockWriter::new();

    let result = handle_leave(state, "Bob".to_string(), &mut writer).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_message_creation() {
    let state = create_state();
    let mut writer = MockWriter::new();

    let result = handle_message(
        state.clone(),
        "Alice".to_string(),
        "Hello".to_string(),
        &mut writer,
    )
    .await;

    assert!(result.is_ok());

    let state = state.lock().await;

    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].user, "Alice");
    assert_eq!(state.messages[0].content, "Hello");
}

#[tokio::test]
async fn test_empty_message_fails() {
    let state = create_state();

    let mut writer = MockWriter::new();

    let result = handle_message(state, "Alice".to_string(), "".to_string(), &mut writer).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_message_ids_increment() {
    let state = create_state();

    let mut writer = MockWriter::new();

    handle_message(
        state.clone(),
        "Alice".to_string(),
        "One".to_string(),
        &mut writer,
    )
    .await
    .unwrap();

    handle_message(
        state.clone(),
        "Alice".to_string(),
        "Two".to_string(),
        &mut writer,
    )
    .await
    .unwrap();

    let state = state.lock().await;

    assert_eq!(state.messages[0].id, 0);
    assert_eq!(state.messages[1].id, 1);
}

#[tokio::test]
async fn test_packet_handler_message() {
    let state = create_state();
    let mut writer = MockWriter::new();
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "Hello".to_string(),
    };

    packet_handler(state.clone(), packet, &mut writer).await;

    let state = state.lock().await;
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.messages[0].user, "Alice");
    assert_eq!(state.messages[0].content, "Hello");
}

#[tokio::test]
async fn test_packet_handler_join() {
    let state = create_state();
    let mut writer = MockWriter::new();
    let packet = Packet::Join("Bob".to_string());

    packet_handler(state.clone(), packet, &mut writer).await;

    let state = state.lock().await;
    assert!(state.connected_users.contains(&"Bob".to_string()));
}

#[tokio::test]
async fn test_packet_handler_leave() {
    let state = create_state();
    let mut writer = MockWriter::new();

    handle_join(state.clone(), "Bob".to_string(), &mut writer)
        .await
        .unwrap();

    let packet = Packet::Leave("Bob".to_string());
    packet_handler(state.clone(), packet, &mut writer).await;

    let state = state.lock().await;
    assert!(!state.connected_users.contains(&"Bob".to_string()));
}

#[tokio::test]
async fn test_empty_sender_message_fails() {
    let state = create_state();

    let mut writer = MockWriter::new();

    let result = handle_message(state, "".to_string(), "Hello".to_string(), &mut writer).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_users_join() {
    let state = create_state();
    let mut writer = MockWriter::new();

    handle_join(state.clone(), "Alice".to_string(), &mut writer)
        .await
        .unwrap();
    handle_join(state.clone(), "Bob".to_string(), &mut writer)
        .await
        .unwrap();
    handle_join(state.clone(), "Charlie".to_string(), &mut writer)
        .await
        .unwrap();

    let state = state.lock().await;

    assert_eq!(state.connected_users.len(), 3);
    assert!(state.connected_users.contains(&"Alice".to_string()));
    assert!(state.connected_users.contains(&"Bob".to_string()));
    assert!(state.connected_users.contains(&"Charlie".to_string()));
}

#[tokio::test]
async fn test_message_timestamp_is_valid() {
    let state = create_state();

    let mut writer = MockWriter::new();

    let (_, timestamp) = handle_message(
        state.clone(),
        "Alice".to_string(),
        "Hello".to_string(),
        &mut writer,
    )
    .await
    .unwrap();
    assert!(timestamp > 0);

    let state = state.lock().await;
    assert_eq!(state.messages[0].timestamp, timestamp);
}

#[tokio::test]
async fn test_send_error_writes_json_and_flushes() {
    let mut writer = MockWriter::new();

    send_error(&mut writer, "ERROR_USER_EXISTS".to_string()).await;

    let output = String::from_utf8(writer.data.clone()).unwrap();
    assert!(output.contains("\"Error\""));
    assert!(output.contains("ERROR_USER_EXISTS"));
    assert!(output.ends_with('\n'));
    assert_eq!(writer.flush_count, 1);
}
