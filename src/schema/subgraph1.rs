use async_graphql::{Context, Object, Result, SimpleObject, InputObject};
use neo4rs::{query, Node, Path, Graph};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(SimpleObject)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct NewUserInput {
    id: String,
    name: String,
    email: String,
}

#[derive(Default)]
pub struct Subgraph1Mutation;

#[Object]
impl Subgraph1Mutation {
    async fn create_user(&self, ctx: &Context<'_>, input: NewUserInput) -> Result<User> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;
        let name = input.name.clone();

        graph
            .run(
                query("CREATE (u:User {id: $id, name: $name, email: $email})")
                    .param("id", input.id.clone())
                    .param("name", name.clone())
                    .param("email", input.email.clone()),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create user: {}", e)))?;

        let mut result = graph
            .execute(
                query("MATCH (u:User { id: $id }) RETURN u")
                    .param("id", input.id),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to query user: {}", e)))?;

        if let Some(row) = result.next().await.unwrap() {
            let node = row.get::<Node>("u").unwrap();
            Ok(User {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                email: node.get("email").unwrap(),
            })
        } else {
            Err(async_graphql::Error::new("User creation failed"))
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;

        let result = graph
            .run(query("MATCH (u:User {id: $id}) DELETE u").param("id", id.clone()))
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to delete user: {}", e)))?;

        let mut result = graph
            .execute(query("MATCH (u:User {id: $id}) RETURN u").param("id", id))
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to check user deletion: {}", e)))?;

        Ok(result.next().await?.is_none())
    }
}

#[derive(Default)]
pub struct Subgraph1Query;

#[Object]
impl Subgraph1Query {
    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let graph = ctx.data::<Arc<Mutex<Graph>>>()?.lock().await;
        let mut result = graph
            .execute(
                query("MATCH (u:User { id: $id }) RETURN u")
                    .param("id", id.clone()),
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to query user: {}", e)))?;

        if let Some(row) = result.next().await.unwrap() {
            let node = row.get::<Node>("u").unwrap();
            Ok(User {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                email: node.get("email").unwrap(),
            })
        } else {
            Err(async_graphql::Error::new("User not found"))
        }
    }
}
