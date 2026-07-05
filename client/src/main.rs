// Imports & Declarations
mod gui;
mod network;
mod packet;

fn main() {
    gui::main();

    std::thread::spawn(|| {
        network::run();
    });
}
