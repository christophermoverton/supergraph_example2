use async_graphql::{Context, Object, Result, SimpleObject};
use crate::db::Neo4jGraph;
use neo4rs::query;
//use tokio_stream::StreamExt;

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
        let graph = ctx.data::<Neo4jGraph>()?.lock().await;
        let mut result = graph
            .execute(query("MATCH (p:Product {id: $id}) RETURN p").param("id", id))
            .await?;
        
        if let Ok(Some(row)) = result.next().await {
            let node = row.get::<neo4rs::Node>("p").unwrap();
            let product = Product {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                price: node.get("price").unwrap(),
            };
            Ok(product)
        } else {
            Err("Product not found".into())
        }
    }
}
