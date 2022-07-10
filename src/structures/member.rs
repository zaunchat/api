use super::{pool, Base, Role};
use chrono::{NaiveDateTime, Utc};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, OpgModel)]
struct MemberRoles(Vec<String>);

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "members")]
pub struct Member {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub nickname: Option<String>,
    #[opg(string)]
    pub joined_at: NaiveDateTime,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub server_id: i64,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[opg(custom = "MemberRoles")]
    pub roles: Vec<i64>,
}

impl Member {
    pub fn new(user_id: i64, server_id: i64) -> Self {
        Self {
            id: user_id,
            nickname: None,
            server_id,
            roles: vec![server_id],
            joined_at: Utc::now().naive_utc(),
        }
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        if self.roles.is_empty() {
            return vec![];
        }

        Role::select()
            .filter("server_id = ANY($1)")
            .bind(self.roles.clone())
            .fetch_all(pool())
            .await
            .unwrap()
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::Server;

        let server = Server::faker().await;
        let member = Self::new(server.owner_id, server.id);

        server.save().await.unwrap();

        member
    }

    #[cfg(test)]
    pub async fn cleanup(self) -> Result<(), crate::utils::Error> {
        use crate::utils::Ref;
        self.server_id.server(None).await?.remove().await?;
        self.id.user().await?.remove().await?;
        Ok(())
    }
}

impl Base for Member {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let member = Member::faker().await.save().await.unwrap();

            let member = Member::select()
                .filter("id = $1 AND server_id = $2")
                .bind(member.id)
                .bind(member.server_id)
                .fetch_one(pool())
                .await
                .expect("Cannot fetch member after it get saved");

            member.cleanup().await.unwrap();
        })
    }

    #[test]
    fn fetch_roles() {
        run(async {
            let mut member = Member::faker().await;
            let role = Role::new("Test".to_string(), member.server_id)
                .save()
                .await
                .unwrap();

            member.roles.push(role.id);

            let member = member.save().await.unwrap();
            let roles = member.fetch_roles().await;

            assert_eq!(roles.len(), 1);

            member.cleanup().await.unwrap();
        });
    }
}
