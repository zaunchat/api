use super::Base;
use crate::utils::snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:sessions | formats_pg:"id:{}::bigint,user_id:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Session {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub token: String,
    #[opg(string)]
    #[serde_as(as = "snowflake::json::ID")]
    pub user_id: u64,
}

impl Base for Session {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Session {
    pub fn new(user_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            token: nanoid!(64).replace('\u{0000}', ""),
            user_id,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::User;

        let user = User::faker();

        user.save().await;

        Self::new(user.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.user_id.user().await.unwrap().delete().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let session = Session::faker().await;

            session.save().await;

            let session = Session::find_one_by_id(session.id).await.unwrap();

            session.cleanup().await;
        })
    }
}
