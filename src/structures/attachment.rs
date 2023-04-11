use super::Base;
use crate::utils::Snowflake;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Model, FromRow, Serialize, Deserialize, Debug, OpgModel, Clone)]
#[ormlite(table = "attachments")]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub height: Option<i32>,
    pub content_type: String,
    pub size: i32,
    #[serde(skip_serializing, default)]
    pub deleted: bool,
}

impl Base for Attachment {}
