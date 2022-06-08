use super::Base;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:messages)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Message {
    pub id: u64,
    pub content: Option<String>,
    pub channel_id: u64,
    pub author_id: u64,
    // pub created_at: u64,
    pub edited_at: Option<u64>, /*
                                TODO:
                                pub embeds: Vec<Embed>
                                pub attachments: Vec<Attachment>
                                pub mentions: Vec<u64>
                                pub replies: Vec<Reply>
                                */
}

impl Base for Message {}

impl Message {
    pub fn new(channel_id: u64, author_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            channel_id,
            author_id,
            ..Default::default()
        }
    }

    pub fn created_at() -> u64 {
        // TODO: extract time from id
        0
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() /* && self.attachments.len() == 0 */
    }
}
