use super::Base;
use crate::utils::snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:invites)]
#[derive(Debug, Serialize, Deserialize, Clone, OpgModel)]
pub struct Invite {
    pub id: u64,
    pub code: String,
    pub uses: u64,
    pub inviter_id: u64,
    pub channel_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<u64>,
}

impl Base for Invite {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Invite {
    pub fn new(inviter_id: u64, channel_id: u64, server_id: Option<u64>) -> Self {
        Self {
            id: snowflake::generate(),
            code: nanoid!(8),
            inviter_id,
            channel_id,
            server_id,
            uses: 0,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::*;

        let user = User::faker();
        let channel = Channel::faker(ChannelTypes::Group).await;

        user.save().await;
        channel.save().await;

        Self::new(user.id, channel.id, None)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;

        self.delete().await;
        self.inviter_id.user().await.unwrap().delete().await;
        self.channel_id.channel(None).await.unwrap().cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;

        let invite = Invite::faker().await;
        invite.cleanup().await;
    }
}
