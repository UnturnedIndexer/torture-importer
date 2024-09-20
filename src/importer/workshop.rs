use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WorkshopItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub authors: Vec<String>,
    pub created_at: Option<Datetime>,
    pub last_updated_at: Datetime,
    pub name: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WorkshopItemUpdate {
    pub authors: Vec<String>,
    pub last_updated_at: Datetime,
    pub name: String,
}
