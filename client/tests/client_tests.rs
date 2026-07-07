use fossil_client::packet::Packet;
use serde_json;


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

    assert_eq!(
        packet,
        Packet::Leave("Alice".to_string())
    );
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