use crate::handler::packet_handler;
use crate::packet::Packet;
use crate::server::ServerState;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

pub fn tcp_listener(state: &mut ServerState) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Creates TcpListener on localhost:7878
    println!("Server running...");
    for stream in listener.incoming() {
        // Listens for oncoming packets and feeds them to packet_handler()
        let stream = stream.unwrap();
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let line = line.unwrap();
            let packet: Packet = serde_json::from_str(&line).unwrap();
            packet_handler(state, packet);
        }
    }
}
