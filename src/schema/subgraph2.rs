use async_graphql::{Context, Object, Result, SimpleObject};
use crate::db::MongoDb;
use mongodb::bson::doc;

#[derive(SimpleObject)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
}

#[derive(Default)]
pub struct Subgraph2Query;

#[Object]
impl Subgraph2Query {
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("products");  // Type annotation here
        let filter = doc! { "id": &id };
        let product_doc = collection.find_one(filter, None).await?;
        
        if let Some(product_doc) = product_doc {
            let product = Product {
                id: product_doc.get_str("id").unwrap().to_string(),
                name: product_doc.get_str("name").unwrap().to_string(),
                price: product_doc.get_f64("price").unwrap(),
            };
            Ok(product)
        } else {
            Err("Product not found".into())
        }
    }
}
