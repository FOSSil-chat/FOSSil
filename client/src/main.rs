use fossil_client::gui;
use fossil_client::network;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use fossil_shared::packet::Packet;  
type Clients = Arc<Mutex<Vec<mpsc::Sender<Packet>>>>;

#[tokio::main]
async fn main() {
    let (_tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        network::run(rx).await;
    });

    gui::launch();
}
