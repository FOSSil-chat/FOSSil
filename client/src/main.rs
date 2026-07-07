use fossil_client::gui;
use fossil_client::network;

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        network::run(tx);
    });

    gui::main(rx);
}
