//use fossil_client::gui;
use fossil_client::network;

use std::sync::mpsc;

fn main() {
    let (tx, _rx) = mpsc::channel(); // Allows communication between threads, temporarily unused

    //std::thread::spawn(move || {
    network::run(tx);
    //});

    /*gui::main(rx);*/
}
