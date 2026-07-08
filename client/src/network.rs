use crate::packet::Packet;
use std::io::{self, Write};
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

pub async fn run(mut _rx: Receiver<String>) {
    let stream = match TcpStream::connect("192.168.0.52:7878").await {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to connect to server");
            return;
        }
    };

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    tokio::spawn(async move {
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // server disconnected :(
                Ok(_) => match serde_json::from_str::<Packet>(&line) {
                    Ok(Packet::Error(error)) => {
                        eprintln!("Server Error: {}", error);
                    }
                    Ok(packet) => {
                        eprintln!("Received: {:?}", packet);
                    }
                    Err(e) => {
                        eprintln!("Invalid packet: {}", e)
                    }
                },
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    });

    let mut name = String::new();

    print!("Enter your name (or !exit to leave chat): "); // Asks use for their name, then sends Packet::Join to the server
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name = name.trim().to_string();

    if name.to_lowercase() == "!exit" {
        println!("Exiting...");
        std::process::exit(0);
    }

    let packet_join = Packet::Join(name.to_string()); // Creates Join packet and converts to JSON
    let json_join = serde_json::to_string(&packet_join).unwrap();
    if writer.write_all(json_join.as_bytes()).await.is_err() {
        eprintln!("Failed to send join packet");
        return;
    }
    if writer.write_all(b"\n").await.is_err() {
        eprintln!("Failed to send newline");
        return;
    }

    loop {
        // Repeatedly asks user for their message, sends packet to server
        let mut content = String::new();
        print!("Message (or !exit to leave chat): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut content)
            .expect("Failed to read line");
        let content = content.trim();
        if content.to_lowercase() == "!exit" {
            println!("Exiting...");
            let packet_leave = Packet::Leave(name.to_string());
            let json_leave = serde_json::to_string(&packet_leave).unwrap();
            if writer.write_all(json_leave.as_bytes()).await.is_err() {
                eprintln!("Failed to send leave packet");
            }
            if writer.write_all(b"\n").await.is_err() {
                eprintln!("Failed to send newline");
            }
            std::process::exit(0);
        }
        if content.is_empty() {
            println!("Message cannot be empty!");
            continue;
        }
        println!("Sending message from {}: '{}'", name, content);

        let packet_send = Packet::Message {
            // Creates packet to send and writes to stream
            user: name.to_string(),
            content: content.to_string(),
        };
        let json_message = serde_json::to_string(&packet_send).unwrap();
        if writer.write_all(json_message.as_bytes()).await.is_err() {
            eprintln!("Failed to send message packet");
            break;
        }
        if writer.write_all(b"\n").await.is_err() {
            eprintln!("Failed to send newline");
            break;
        }
    }
}
