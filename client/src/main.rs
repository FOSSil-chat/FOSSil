// Imports & Declarations
mod packet;
mod gui;
mod network;

fn main() { 
    gui::main();

    std::thread::spawn(|| {
        network::run();
    });
}