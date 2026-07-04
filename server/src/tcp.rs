use std::net::TcpListener;
use std::io::{BufRead, BufReader};
use crate::handler::packet_handler;
use crate::packet::Packet;
use crate::server::ServerState;


pub fn tcp_listener(state: &mut ServerState) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(stream);
        for line in reader.lines() {
            let line = line.unwrap();
            let packet: Packet = serde_json::from_str(&line).unwrap();
            packet_handler(state, packet);
        }
    }
}