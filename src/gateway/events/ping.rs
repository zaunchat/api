use crate::gateway::*;
use crate::utils::Error;
use std::sync::Arc;

pub async fn run(client: Arc<SocketClient>, conn: Sender) -> Result<(), Error> {
    client.send(&conn, Payload::Pong).await?;
    Ok(())
}
