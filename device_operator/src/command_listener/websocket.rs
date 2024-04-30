use anyhow::{Context, Result};
use async_trait::async_trait;
use log::warn;
use tokio_tungstenite::tungstenite::{connect, Message};
use url::Url;

use crate::commands::router::command_router;

use super::Listener;

pub struct WebsocketListener;

#[async_trait]
impl Listener for WebsocketListener {
    async fn listen(&self) -> Result<()> {
        let websocket_addr = "ws://127.0.0.1:3000";
        let url = Url::parse(websocket_addr)?;
        let (mut socket, _) = connect(url).context("Unable to connect to websocket")?;

        loop {
            let msg = socket.read().context("Unable to read message");
            let _ = match msg?.into_text() {
                Ok(data) => match command_router(&data).await {
                    Ok(value) => {
                        let _ = socket
                            .send(Message::text(value))
                            .inspect_err(|e| warn!("Error sending response {e:?}"));
                    }
                    Err(err) => {
                        warn!("Error: {err:?}");
                    }
                },
                Err(err) => {
                    warn!("Error parsing message {err:?}");
                }
            };
        }
    }
}
