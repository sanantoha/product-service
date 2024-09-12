use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub currency: String,
    pub price: i64,
}