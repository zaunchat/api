use super::*;
use crate::utils::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, Arguments, FromRow};

#[serde_as]
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, OpgModel)]
pub struct Bot {
    pub id: Snowflake,
    pub username: String,
    pub owner_id: Snowflake,
    pub verified: bool,
}

impl Bot {
    pub fn new(username: String, owner_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            username,
            owner_id,
            verified: false,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let owner = User::faker();
        let bot = Self::new("Ghost Bot".to_string(), owner.id);

        owner.insert().await?;

        Ok(bot)
    }
}

impl Base<'_, Snowflake> for Bot {
    fn id(&self) -> Snowflake {
        self.id
    }

    fn table_name() -> &'static str {
        "bots"
    }

    fn fields(&self) -> (Vec<&str>, PgArguments) {
        let mut values = PgArguments::default();

        values.add(self.id);
        values.add(&self.username);
        values.add(self.owner_id);
        values.add(self.verified);

        (vec!["id", "username", "owner_id", "verified"], values)
    }
}
