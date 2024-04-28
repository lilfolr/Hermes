use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Listener {
    async fn listen(&self) -> Result<()>;
}

pub mod rabbitmq;
pub mod websocket;
