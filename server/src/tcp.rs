use crate::handler::packet_handler;
use crate::server::ServerState;
use fossil_shared::packet::Packet;
use socket2::{Socket, TcpKeepalive};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

fn set_keepalive(stream: TcpStream) -> std::io::Result<TcpStream> {
    let socket = Socket::from(stream.into_std()?);

    socket.set_keepalive(true)?;

    socket.set_tcp_keepalive(
        &TcpKeepalive::new()
            .with_time(Duration::from_secs(60))
            .with_interval(Duration::from_secs(10)),
    )?;

    socket.set_nonblocking(true)?;

    TcpStream::from_std(socket.into())
}

pub async fn tcp_listener(state: Arc<Mutex<ServerState>>) {
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap(); // Creates TcpListener on localhost:7878

    println!("Server running...");
    loop {
        // Listens for oncoming packets and feeds them to packet_handler()
        let (stream, _) = listener.accept().await.unwrap();

        let stream = match set_keepalive(stream) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to set keepalive: {}", e);
                continue;
            }
        };
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let (reader, writer) = stream.into_split();
            let mut buf_reader = BufReader::new(reader);
            let mut buf_writer = writer;
            let mut line = String::new();
            let mut username: Option<String> = None;

            loop {
                line.clear();

                match buf_reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Ok(packet) = serde_json::from_str::<Packet>(&line) {
                            if let Packet::Join(name) = &packet {
                                username = Some(name.clone());
                            }

                            packet_handler(state.clone(), packet, &mut buf_writer).await;
                        }
                    }
                    Err(_) => break,
                }
            }

            // Connection closed/crashed, fake a Leave packet
            if let Some(name) = username {
                packet_handler(state.clone(), Packet::Leave(name), &mut buf_writer).await;
            }
        });
    }
}

pub async fn send_error<W: AsyncWriteExt + Unpin>(writer: &mut W, error_type: String) {
    let error_packet = Packet::Error(error_type);

    if let Ok(error_json) = serde_json::to_string(&error_packet) {
        let _ = writer.write_all(error_json.as_bytes()).await;
        let _ = writer.write_all(b"\n").await;
        let _ = writer.flush().await;
    }
}

