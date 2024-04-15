mod command_listener;
mod commands;
mod kubernetes;

use anyhow::Ok;
use env_logger::Builder;
use log::{info, warn};

use crate::command_listener::websocket_listener::listener;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Set minimum log level to Info
        .init();

    info!("Starting");

    let websocket_addr = "ws://127.0.0.1:1234/";
    let _ = listener(websocket_addr)
        .await
        .or_else(|e| {
            warn!("Listener exited: {}", e);
            return Ok(());
        });

    info!("Exiting");
}
