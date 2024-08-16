use async_graphql::{Context, Object, Result};
use redis::AsyncCommands;
use crate::models::Product;
use serde_json;
//use uuid::Uuid;


#[derive(Default)]
pub struct ProductQuery;

#[Object]
impl ProductQuery {
    async fn get_all_products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let redis_client = ctx.data::<redis::Client>()?;
        let mut con = redis_client.get_async_connection().await?;
        let products: Vec<String> = con.smembers("products").await?;
        
        let products: Vec<Product> = products
            .into_iter()
            .filter_map(|p| serde_json::from_str(&p).ok())
            .collect();

        Ok(products)
    }
}

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductMutation {
    async fn add_product(&self, ctx: &Context<'_>, name: String, price: f64) -> Result<Product> {
        let redis_client = ctx.data::<redis::Client>()?;
        let mut con = redis_client.get_async_connection().await?;

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            price,
        };

        let product_json = serde_json::to_string(&product)?;
        con.sadd("products", product_json).await?;
        
        Ok(product)
    }
}
