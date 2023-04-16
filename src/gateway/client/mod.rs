pub mod config;
pub mod connection;
pub mod handler;
mod state;
use crate::gateway::Payload;
use crate::structures::User;
use config::*;
use connection::Sender;
use dashmap::DashMap;
use fred::clients::SubscriberClient;
use state::SocketClientState;

pub struct SocketClient {
    pub state: SocketClientState,
    pub config: SocketClientConfig,
    pub connections: DashMap<String, Sender>,
    pub subscriptions: SubscriberClient,
}

impl SocketClient {
    pub fn new(user: User, config: SocketClientConfig, subscriptions: SubscriberClient) -> Self {
        Self {
            state: SocketClientState::new(user),
            connections: DashMap::new(),
            subscriptions,
            config,
        }
    }

    pub async fn broadcast(&self, payload: Payload) -> Result<(), axum::Error> {
        let payload = self.config.encode(payload);

        log::debug!("Payload encoded: {payload:?}");

        log::debug!("Connections found: {}", self.connections.len());

        for conn in &self.connections {
            log::debug!("Sending payload bytes...");
            conn.value().send(payload.clone()).await?;
            log::debug!("Sent payload to connection node");
        }

        Ok(())
    }

    pub async fn send(&self, connection: &Sender, payload: Payload) -> Result<(), axum::Error> {
        connection.send(self.config.encode(payload)).await
    }
}
