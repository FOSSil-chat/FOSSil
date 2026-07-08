use fossil_client::network::{describe_packet, parse_packet_line, send_packet_line};
use fossil_client::packet::Packet;
use serde_json;

struct MockWriter {
    data: Vec<u8>,
    flush_count: usize,
}

impl MockWriter {
    fn new() -> Self {
        Self {
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

#[test]
fn test_join_packet_creation() {
    let packet = Packet::Join("Alice".to_string());

    match packet {
        Packet::Join(name) => {
            assert_eq!(name, "Alice");
        }
        _ => panic!("Expected Join packet"),
    }
}

#[test]
fn test_leave_packet_creation() {
    let packet = Packet::Leave("Alice".to_string());

    assert_eq!(packet, Packet::Leave("Alice".to_string()));
}

#[test]
fn test_message_packet_creation() {
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "Hello".to_string(),
    };

    assert_eq!(
        packet,
        Packet::Message {
            user: "Alice".to_string(),
            content: "Hello".to_string(),
        }
    );
}

#[test]
fn test_packet_serialization() {
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "Hello".to_string(),
    };

    let json = serde_json::to_string(&packet).unwrap();

    assert!(json.contains("Alice"));
    assert!(json.contains("Hello"));
}

#[test]
fn test_packet_deserialization() {
    let json = r#"{
        "Message": {
            "user": "Alice",
            "content": "Hello"
        }
    }"#;

    let packet: Packet = serde_json::from_str(json).unwrap();

    assert_eq!(
        packet,
        Packet::Message {
            user: "Alice".to_string(),
            content: "Hello".to_string(),
        }
    );
}

#[test]
fn test_join_packet_serialization() {
    let packet = Packet::Join("Alice".to_string());
    let json = serde_json::to_string(&packet).unwrap();

    assert!(json.contains("Alice"));
    let deserialized: Packet = serde_json::from_str(&json).unwrap();
    assert_eq!(packet, deserialized);
}

#[test]
fn test_leave_packet_serialization() {
    let packet = Packet::Leave("Bob".to_string());
    let json = serde_json::to_string(&packet).unwrap();

    assert!(json.contains("Bob"));
    let deserialized: Packet = serde_json::from_str(&json).unwrap();
    assert_eq!(packet, deserialized);
}

#[test]
fn test_message_packet_with_special_characters() {
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "Hello, how are you? 🎉".to_string(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    let deserialized: Packet = serde_json::from_str(&json).unwrap();

    assert_eq!(packet, deserialized);
}

#[test]
fn test_empty_message_content() {
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: "".to_string(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    let deserialized: Packet = serde_json::from_str(&json).unwrap();

    assert_eq!(packet, deserialized);
}

#[test]
fn test_long_username() {
    let long_name = "a".repeat(256);
    let packet = Packet::Join(long_name.clone());

    match packet {
        Packet::Join(name) => {
            assert_eq!(name, long_name);
        }
        _ => panic!("Expected Join packet"),
    }
}

#[test]
fn test_error_packet_is_formatted_for_terminal() {
    let packet = Packet::Error("ERROR_USER_EXISTS".to_string());

    assert_eq!(describe_packet(&packet), "Server Error: ERROR_USER_EXISTS");
}

#[test]
fn test_packet_line_is_parsed() {
    let line = r#"{"Error":"ERROR_USER_EXISTS"}"#;
    let packet = parse_packet_line(line).unwrap();

    assert_eq!(packet, Packet::Error("ERROR_USER_EXISTS".to_string()));
}

#[tokio::test]
async fn test_send_packet_line_writes_and_flushes() {
    let mut writer = MockWriter::new();
    let packet = Packet::Join("Alice".to_string());

    send_packet_line(&mut writer, &packet).await.unwrap();

    let output = String::from_utf8(writer.data).unwrap();
    assert!(output.contains("Alice"));
    assert!(output.ends_with('\n'));
    assert_eq!(writer.flush_count, 1);
}

#[test]
fn test_long_message_content() {
    let long_content = "Lorem ipsum dolor sit amet, ".repeat(100);
    let packet = Packet::Message {
        user: "Alice".to_string(),
        content: long_content.clone(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    let deserialized: Packet = serde_json::from_str(&json).unwrap();

    assert_eq!(packet, deserialized);
}

#[test]
fn test_multiple_packets_serialization() {
    let packets = vec![
        Packet::Join("Alice".to_string()),
        Packet::Message {
            user: "Alice".to_string(),
            content: "Hello".to_string(),
        },
        Packet::Leave("Alice".to_string()),
    ];

    for packet in packets {
        let json = serde_json::to_string(&packet).unwrap();
        let deserialized: Packet = serde_json::from_str(&json).unwrap();
        assert_eq!(packet, deserialized);
    }
}

#[test]
fn test_packet_with_unicode() {
    let packet = Packet::Message {
        user: "用户".to_string(),
        content: "你好世界".to_string(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    let deserialized: Packet = serde_json::from_str(&json).unwrap();

    assert_eq!(packet, deserialized);
}
