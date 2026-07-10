use fossil_server::server::ServerState;
use fossil_server::tcp::tcp_listener;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(ServerState {
        // Creates server state and wraps in arc mutex
        connected_users: Vec::new(),
        messages: Vec::new(),
        next_message_id: 0,
    }));

    tcp_listener(state).await;
}
