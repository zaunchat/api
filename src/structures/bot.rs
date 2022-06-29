use super::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:bots | formats_pg:"id:{}::bigint,owner_id:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, OpgModel)]
pub struct Bot {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub username: String,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub owner_id: u64,
    pub verified: bool,
}

#[async_trait]
impl Base for Bot {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Bot {
    pub fn new(username: String, owner_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            username,
            owner_id,
            verified: false,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let owner = User::faker();

        owner.save().await;

        Self::new("Ghost Bot".to_string(), owner.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.owner_id.user().await.unwrap().delete().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let bot = Bot::faker().await;

            bot.save().await;

            let bot = Bot::find_one_by_id(bot.id).await.unwrap();

            bot.cleanup().await;
        })
    }
}
