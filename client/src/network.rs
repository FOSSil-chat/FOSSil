use crate::packet::Packet;
use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub fn run(_tx: Sender<String>) {
    // Remove _ when communicating between GUI and network.rs
    let mut stream = TcpStream::connect("fossil.simarpreetsingh.org:7878").unwrap();

    let mut name = String::new();

    print!("Enter your name: "); // Asks use for their name, then sends Packet::Join to the server
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let packet_join = Packet::Join(name.to_string());
    let json_join = serde_json::to_string(&packet_join).unwrap();
    stream.write_all(json_join.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
    loop {
        // Repeatedly asks user for their message, sends packet to server
        let mut content = String::new();
        print!("Message: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut content)
            .expect("Failed to read line");
        let content = content.trim();
        if content.is_empty() {
            println!("Message cannot be empty!");
            continue;
        }
        println!("Sending message from {}: '{}'", name, content);

        let packet_send = Packet::Message {
            user: name.to_string(),
            content: content.to_string(),
        };
        let json_message = serde_json::to_string(&packet_send).unwrap();
        stream.write_all(json_message.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();
    }
}
