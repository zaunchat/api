use crate::gateway::SocketClient;
use crate::utils::Snowflake;
use dashmap::DashMap;
use std::sync::Arc;

pub struct WebSocketState {
    pub clients: DashMap<Snowflake, Arc<SocketClient>>,
}

impl Default for WebSocketState {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketState {
    pub fn new() -> Self {
        Self {
            clients: DashMap::new(),
        }
    }
}
