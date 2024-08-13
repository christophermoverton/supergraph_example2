use async_graphql::{Context, Object, Result, SimpleObject};
use crate::db::MongoDb;
use mongodb::bson::doc;

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
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("users");  // Type annotation here
        let filter = doc! { "id": &id };
        let user_doc = collection.find_one(filter, None).await?;
        
        if let Some(user_doc) = user_doc {
            let user = User {
                id: user_doc.get_str("id").unwrap().to_string(),
                name: user_doc.get_str("name").unwrap().to_string(),
                email: user_doc.get_str("email").unwrap().to_string(),
            };
            Ok(user)
        } else {
            Err("User not found".into())
        }
    }
}
