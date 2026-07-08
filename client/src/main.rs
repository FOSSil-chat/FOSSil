use fossil_client::gui;
use fossil_client::network;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (_tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        network::run(rx).await;
    });

    match gui::main().await {
        Ok(_) => (),
        Err(_) => println!("Error in gui::main()"),
    }
}
