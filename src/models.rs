use serde::{Serialize, Deserialize};
use async_graphql::SimpleObject;

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}
