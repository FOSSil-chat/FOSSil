// Imports & Declarations
use fossil_client::gui;
use fossil_client::network;

fn main() {
    gui::main();

    std::thread::spawn(|| {
        network::run();
    });
}
