use super::*;
use crate::utils::Snowflake;
use chrono::NaiveDateTime;
use ormlite::model::*;
use ormlite::types::Json;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "messages")]
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
            edited_at: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.attachments.0.is_empty()
    }
}

impl Base for Message {}
