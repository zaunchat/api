use super::Base;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:messages)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Message {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub channel_id: u64,
    pub author_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<u64>, /*
                                TODO:
                                pub embeds: Vec<Embed>
                                pub attachments: Vec<Attachment>
                                pub mentions: Vec<u64>
                                pub replies: Vec<Reply>
                                */
}

impl Base for Message {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Message {
    pub fn new(channel_id: u64, author_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            channel_id,
            author_id,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() /* && self.attachments.len() == 0 */
    }
}
