use anyhow::Result;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;

pub async fn listener(websocket_addr: &str, message_callback: &dyn Fn(&str) -> Result<()>) -> Result<()> {
    let url = Url::parse(&websocket_addr)?;
    let (ws_stream, _) = connect_async(url).await?;

    let (mut _write, read) = ws_stream.split();

    let read_future = read.for_each(|message| async {
        println!("Got message");
        let data = message.unwrap().into_text().unwrap();
        println!(" > {}", data);
        message_callback(&data);
    });

    read_future.await;

    println!("Socket Exit");

    Ok(())
}
