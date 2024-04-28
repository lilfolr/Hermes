use std::env;

use super::Listener;
use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultBlockingConsumer,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use log::info;
use tokio::sync::Notify;

pub struct RabbitmqListener;

fn get_env_var(key: &str) -> String {
    env::var(key)
        .context(format!("Environment variable {} not set!", key))
        .unwrap()
}

#[async_trait]
impl Listener for RabbitmqListener {
    async fn listen(self: &Self) -> Result<()> {
        let host = get_env_var("RABBITMQ_HOST");
        let port = u16::from_str_radix(get_env_var("RABBITMQ_PORT").as_str(), 10)
            .context(format!("Invalid port {}", get_env_var("RABBITMQ_PORT")))
            .unwrap();
        let username = get_env_var("RABBITMQ_USERNAME");
        let password = get_env_var("RABBITMQ_PASSWORD");
        let queue_name = get_env_var("RABBITMQ_QUEUE");
        let consumer_tag = ""; // empty = server generated

        info!("Connecting to {}:{} as user {}", host, port, username);
        let connection = Connection::open(&OpenConnectionArguments::new(
            host.as_str(),
            port,
            username.as_str(),
            password.as_str(),
        ))
        .await
        .unwrap();

        connection
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();
        let channel = connection.open_channel(None).await.unwrap();

        info!("Creating queue {}", queue_name);
        let (queue_name, _, _) = channel
            .queue_declare(
                QueueDeclareArguments::default()
                    .queue(queue_name)
                    .auto_delete(false)
                    .passive(true) // Don't throw if queue already exists
                    .durable(true)
                    .finish(),
            )
            .await
            .unwrap()
            .unwrap();
        let args = BasicConsumeArguments::new(&queue_name, consumer_tag)
            .exclusive(true)
            .finish();

        channel
            .basic_consume_blocking(DefaultBlockingConsumer::new(args.no_ack), args)
            .await
            .unwrap();

        // Wait forever to keep consuming
        info!("Consuming messages in queue {}", queue_name);
        let guard = Notify::new();
        guard.notified().await;

        Ok(())
    }
}
