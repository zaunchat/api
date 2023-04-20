use super::*;
use crate::utils::Snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, Arguments, FromRow};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, OpgModel)]
pub struct Session {
    pub id: Snowflake,
    #[serde(skip)]
    pub token: String,
    #[serde(skip)]
    pub user_id: Option<Snowflake>,
}

impl Session {
    pub fn new(user_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            token: nanoid!(64),
            user_id: Some(user_id),
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let user = User::faker();
        let session = Self::new(user.id);

        user.insert().await?;

        Ok(session)
    }
}

impl Base<'_, Snowflake> for Session {
    fn id(&self) -> Snowflake {
        self.id
    }

    fn table_name() -> &'static str {
        "sessions"
    }

    fn fields(&self) -> (Vec<&str>, PgArguments) {
        let mut values = PgArguments::default();

        values.add(self.id);
        values.add(&self.token);
        values.add(self.user_id);

        (vec!["id", "token", "user_id"], values)
    }
}
