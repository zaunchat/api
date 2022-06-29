use super::Base;
use crate::utils::snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:invites | formats_pg:"id:{}::bigint,channel_id:{}::bigint,inviter_id:{}::bigint,server_id:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, OpgModel)]
pub struct Invite {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub code: String,
    pub uses: u32,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub inviter_id: u64,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub channel_id: u64,
    #[opg(string, nullable)]
    #[serde_as(as = "Option<snowflake::json::ID>")]
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
            code: nanoid!(8).replace('\u{0000}', ""),
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
        let channel = self.channel_id.channel(None).await.unwrap();

        channel.cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let invite = Invite::faker().await;

            invite.save().await;

            let invite = Invite::find_one_by_id(invite.id).await.unwrap();

            invite.cleanup().await;
        })
    }
}
