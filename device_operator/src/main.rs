mod command_listener;
mod commands;
mod kubernetes;

use core::panic;
use std::env;

use anyhow::Context;
use command_listener::{rabbitmq::RabbitmqListener, websocket::WebsocketListener, Listener};
use env_logger::Builder;
use log::info;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Set minimum log level to Info
        .init();

    info!("Starting");
    let listener = get_listener();
    listener
        .listen()
        .await
        .context("Websocket listener exited")
        .unwrap();
}

fn get_listener() -> Box<dyn Listener> {
    let listener_type = env::var("LISTENER").ok().map(|s| s.to_uppercase());
    match listener_type.as_ref().map(String::as_ref) {
        None => {
            panic!("No lisener type defined! Set the LISTENER env var");
        }
        Some("RABBITMQ") => Box::new(RabbitmqListener {}),
        Some("WEBSOCKET") => Box::new(WebsocketListener {}),
        Some(s) => {
            panic!("Unknown listener type {}", s);
        }
    }
}
