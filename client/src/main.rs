use fossil_client::gui;
use fossil_client::network;
use fossil_shared::packet::Packet;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
type _Clients = Arc<Mutex<Vec<mpsc::Sender<Packet>>>>;

#[tokio::main]
async fn main() {
    let (_tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        network::run(rx).await;
    });

    gui::launch();
}
