use anyhow::{Context, Ok, Result};
use futures_util::StreamExt;
use log::{info, warn};
use tokio_tungstenite::connect_async;
use url::Url;

use crate::commands::router::command_router;

pub async fn listener(
    websocket_addr: &str
) -> Result<()> {
    let url = Url::parse(&websocket_addr)?;
    let (ws_stream, _) = connect_async(url).await?;

    let (mut _write, read) = ws_stream.split();

    let read_future = read.for_each(|message| async {
        info!("Got message");
        let data = message.unwrap().into_text().unwrap();
        info!(" > {}", data);
        let message_result = command_router(&data).await;
        if let Err(err) = message_result.context("Error invoking mesage processor") {
            warn!("Error: {}", err);
        }
    });

    read_future.await;

    info!("Socket Exit");

    Ok(())
}
