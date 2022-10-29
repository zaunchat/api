use crate::gateway::SocketClient;
use dashmap::DashMap;
use std::sync::Arc;

pub struct WebSocketState {
    pub clients: DashMap<i64, Arc<SocketClient>>,
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
