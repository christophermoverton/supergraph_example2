use async_graphql::{Context, Object, Result, SimpleObject};
use crate::db::Neo4jGraph;
use neo4rs::query;
//use tokio_stream::StreamExt;

#[derive(SimpleObject)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Default)]
pub struct Subgraph1Query;

#[Object]
impl Subgraph1Query {
    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let graph = ctx.data::<Neo4jGraph>()?.lock().await;
        let mut result = graph
            .execute(query("MATCH (u:User {id: $id}) RETURN u").param("id", id))
            .await?;
        
        if let Ok(Some(row)) = result.next().await {
            let node = row.get::<neo4rs::Node>("u").unwrap();
            let user = User {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                email: node.get("email").unwrap(),
            };
            Ok(user)
        } else {
            Err("User not found".into())
        }
    }
}
