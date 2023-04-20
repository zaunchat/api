use super::Base;
use crate::utils::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, Arguments, FromRow};

#[serde_as]
#[derive(FromRow, Serialize, Deserialize, Debug, OpgModel, Clone)]
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

impl Base<'_, Snowflake> for Attachment {
    fn id(&self) -> Snowflake {
        self.id
    }

    fn table_name() -> &'static str {
        "attachments"
    }

    fn fields(&self) -> (Vec<&str>, PgArguments) {
        let mut values = PgArguments::default();

        values.add(self.id);
        values.add(&self.filename);
        values.add(self.width);
        values.add(self.height);
        values.add(&self.content_type);
        values.add(self.size);
        values.add(self.deleted);

        (
            vec![
                "id",
                "filename",
                "width",
                "height",
                "content_type",
                "size",
                "deleted",
            ],
            values,
        )
    }
}
