use super::*;
use crate::utils::Snowflake;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{postgres::PgArguments, Arguments, FromRow};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, OpgModel)]
pub struct Message {
    pub id: Snowflake,
    pub content: Option<String>,
    pub attachments: Json<Vec<Attachment>>,
    pub channel_id: Snowflake,
    pub author_id: Snowflake,
    pub edited_at: Option<NaiveDateTime>,
}

impl Message {
    pub fn new(channel_id: Snowflake, author_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            content: None,
            channel_id,
            author_id,
            attachments: Json(vec![]),
            edited_at: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.attachments.0.is_empty()
    }
}

impl Base<'_, Snowflake> for Message {
    fn id(&self) -> Snowflake {
        self.id
    }

    fn table_name() -> &'static str {
        "messages"
    }

    fn fields(&self) -> (Vec<&str>, sqlx::postgres::PgArguments) {
        let mut values = PgArguments::default();

        values.add(self.id);
        values.add(&self.content);
        values.add(&self.attachments);
        values.add(self.channel_id);
        values.add(self.author_id);
        values.add(self.edited_at);

        (
            vec!["id", "content", "attachments", "channel_id", "author_id", "edited_at"],
            values,
        )
    }
}
