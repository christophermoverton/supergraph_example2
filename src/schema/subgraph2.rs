use async_graphql::{Context, Object, Result, SimpleObject, InputObject};
use neo4rs::{query, Node, Path, Graph};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(SimpleObject)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
}

#[derive(InputObject)]
pub struct NewProductInput {
    id: String,
    name: String,
    price: f64,
}

#[derive(Default)]
pub struct Subgraph2Mutation;

#[Object]
impl Subgraph2Mutation {
    async fn create_product(&self, ctx: &Context<'_>, input: NewProductInput) -> Result<Product> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;
        let name = input.name.clone();

        graph
            .run(
                query("CREATE (p:Product {id: $id, name: $name, price: $price})")
                    .param("id", input.id.clone())
                    .param("name", name.clone())
                    .param("price", input.price),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create product: {}", e)))?;

        let mut result = graph
            .execute(
                query("MATCH (p:Product { id: $id }) RETURN p")
                    .param("id", input.id),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to query product: {}", e)))?;

        if let Some(row) = result.next().await.unwrap() {
            let node = row.get::<Node>("p").unwrap();
            Ok(Product {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                price: node.get("price").unwrap(),
            })
        } else {
            Err(async_graphql::Error::new("Product creation failed"))
        }
    }

    async fn delete_product(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;

        let result = graph
            .run(query("MATCH (p:Product {id: $id}) DELETE p").param("id", id.clone()))
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to delete product: {}", e)))?;

        let mut result = graph
            .execute(query("MATCH (p:Product {id: $id}) RETURN p").param("id", id))
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to check product deletion: {}", e)))?;

        Ok(result.next().await?.is_none())
    }
}

#[derive(Default)]
pub struct Subgraph2Query;

#[Object]
impl Subgraph2Query {
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;
        let mut result = graph
            .execute(
                query("MATCH (p:Product { id: $id }) RETURN p")
                    .param("id", id.clone()),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to query product: {}", e)))?;

        if let Some(row) = result.next().await.unwrap() {
            let node = row.get::<Node>("p").unwrap();
            Ok(Product {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                price: node.get("price").unwrap(),
            })
        } else {
            Err(async_graphql::Error::new("Product not found"))
        }
    }
}
