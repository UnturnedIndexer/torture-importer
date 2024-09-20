use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Thing};
use torture_parser::parser::assets::{Rarity, Type};

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub name: String,
    pub description: String,
    pub guid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Object>,
    pub rarity: Rarity,
    pub r#type: Type,
    pub non_unique_id: u16,
    pub inserted_at: Datetime,
    pub workshop_item: Thing,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetUpdate {
    pub name: String,
    pub description: String,
    pub guid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Object>,
    pub rarity: Rarity,
    pub r#type: Type,
    pub non_unique_id: u16,
}
