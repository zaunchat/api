use serde::{Deserialize, Serialize};


#[crud_table(table_name:bots)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: i64,
}

impl Bot {
    
}