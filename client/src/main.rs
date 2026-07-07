use fossil_client::gui;
use fossil_client::network;

use std::sync::mpsc;

fn main() {
    gui::main();

    let (tx, _rx) = mpsc::channel(); // Allows communication between threads, temporarily unused

    //std::thread::spawn(move || {
    network::run(tx);
    //});
}
