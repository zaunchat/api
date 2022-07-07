use super::*;
use crate::utils::snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "invites")]
pub struct Invite {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub code: String,
    pub uses: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub inviter_id: i64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub channel_id: i64,
    #[opg(string, nullable)]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub server_id: Option<i64>,
}

impl Invite {
    pub fn new(inviter_id: i64, channel_id: i64, server_id: Option<i64>) -> Self {
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
        let invite = Self::new(user.id, channel.id, None);

        user.insert(pool()).await.unwrap();
        channel.insert(pool()).await.unwrap();

        invite
    }

    #[cfg(test)]
    pub async fn cleanup(self) {
        use crate::utils::Ref;
        self.inviter_id
            .user()
            .await
            .unwrap()
            .delete(pool())
            .await
            .unwrap();
        self.channel_id.channel(None).await.unwrap().cleanup().await;
    }
}

impl Base for Invite {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let invite = Invite::faker().await;
            let invite = invite.insert(pool()).await.unwrap();
            let invite = Invite::get_one(invite.id, pool()).await.unwrap();

            invite.cleanup().await;
        });
    }
}
