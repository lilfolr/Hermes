
mod command_listener;
mod commands;

use crate::command_listener::websocket_listener::listener;
use crate::commands::router::command_router;

#[tokio::main]
async fn main() {
    println!("Starting");
    let websocket_addr = "ws://127.0.0.1:1234/";

    listener(websocket_addr, &command_router).await;

    println!("Exiting");
}
