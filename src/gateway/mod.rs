mod client;
mod events;
mod payload;
mod server;

use axum::extract::ws::WebSocket;
use futures::stream::SplitStream;

pub type Receiver = SplitStream<WebSocket>;
pub use client::{config::*, connection::Sender, SocketClient};
pub use payload::*;
pub use server::upgrade;
