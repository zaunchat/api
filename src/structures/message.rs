use super::Base;
use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:messages)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Message {
    pub id: i64,
    pub content: Option<String>,
    pub channel_id: i64,
    pub author_id: i64,
    // pub created_at: i64,
    pub edited_at: Option<i64>, /*
                                TODO:
                                pub embeds: Vec<Embed>
                                pub attachments: Vec<Attachment>
                                pub mentions: Vec<i64>
                                pub replies: Vec<Reply>
                                */
}

impl Base for Message {}

impl Message {
    pub fn new(channel_id: i64, author_id: i64) -> Self {
        Self {
            id: generate_id(),
            channel_id,
            author_id,
            ..Default::default()
        }
    }

    pub fn created_at() -> i64 {
        // TODO: extract time from id
        0
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() /* && self.attachments.len() == 0 */
    }
}
