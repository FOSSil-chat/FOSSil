use crate::handler::packet_handler;
use crate::packet::Packet;
use crate::server::ServerState;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn tcp_listener(state: Arc<Mutex<ServerState>>) {
    let listener = TcpListener::bind("192.168.0.52:7878").await.unwrap(); // Creates TcpListener on localhost:7878

    println!("Server running...");
    loop {
        // Listens for oncoming packets and feeds them to packet_handler()
        let (stream, _) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let (reader, writer) = stream.into_split();
            let mut buf_reader = BufReader::new(reader);
            let mut buf_writer = writer;
            let mut line = String::new();
            loop {
                line.clear();
                match buf_reader.read_line(&mut line).await {
                    Ok(0) => break, // Connection closed
                    Ok(_) => {
                        if let Ok(packet) = serde_json::from_str::<Packet>(&line) {
                            packet_handler(state.clone(), packet, &mut buf_writer).await;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}

pub async fn send_error<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    error_type: String,
) {
    let error_packet = Packet::Error(error_type);

    if let Ok(error_json) = serde_json::to_string(&error_packet) {
        let _ = writer.write_all(error_json.as_bytes()).await;
        let _ = writer.write_all(b"\n").await;
    }
}