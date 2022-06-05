use serde::{Deserialize, Serialize};


#[crud_table(table_name:messages)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    pub content: Option<String>,
    pub channel_id: i64,
    pub author_id: i64,
    pub created_at: i64,
    pub edited_at: Option<i64>
    /*
    TODO:
    pub embeds: Vec<Embed>
    pub attachments: Vec<Attachment>
    pub mentions: Vec<i64>
    pub replies: Vec<Reply>
    */
}


impl Message {
    // TODO:
    pub fn is_empty(&self) -> bool {
        false
    }

    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}