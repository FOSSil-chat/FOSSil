use fossil_client::gui;
use fossil_client::network;

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        network::run(tx);
    });

    match gui::main() {
        Ok(_) => (),
        Err(_) => println!("Error in gui::main()"),
    }

    drop(rx);
}
