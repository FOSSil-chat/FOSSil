use crate::handler::packet_handler;
use crate::packet::Packet;
use crate::server::ServerState;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::net::TcpStream;


pub fn tcp_listener(state: Arc<Mutex<ServerState>>) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Creates TcpListener on localhost:7878

    println!("Server running...");
    for stream in listener.incoming() {
        // Listens for oncoming packets and feeds them to packet_handler()
        let stream = stream.unwrap();
        let state = Arc::clone(&state);
        std::thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                let line = line.unwrap();
                let packet: Packet = serde_json::from_str(&line).unwrap();
                packet_handler(state.clone(), packet);
            }
        });
    }
}

pub fn send_error(error_type: String) {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let error_packet = Packet::Join(error_type.to_string());
    let error_json = serde_json::to_string(&error_packet).unwrap();
    stream.write_all(error_json.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
}
