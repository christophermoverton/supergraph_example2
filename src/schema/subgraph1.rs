use async_graphql::{Context, Object, Result, SimpleObject, InputObject};
use crate::db::MongoDb;
use mongodb::bson::{doc, Bson};
use mongodb::options::UpdateOptions;

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
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("users");

        let user_doc = doc! {
            "id": &input.id,
            "name": &input.name,
            "email": &input.email,
        };

        collection.insert_one(user_doc.clone(), None).await?;
        let user = User {
            id: input.id,
            name: input.name,
            email: input.email,
        };
        Ok(user)
    }

    async fn update_user(&self, ctx: &Context<'_>, id: String, name: Option<String>, email: Option<String>) -> Result<User> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("users");

        let mut update_doc = doc! {};
        if let Some(name) = name {
            update_doc.insert("name", Bson::String(name));
        }
        if let Some(email) = email {
            update_doc.insert("email", Bson::String(email));
        }

        let update_result = collection.update_one(doc! {"id": &id}, doc! {"$set": update_doc}, UpdateOptions::builder().upsert(true).build()).await?;
        
        if update_result.matched_count > 0 {
            let updated_user_doc = collection.find_one(doc! {"id": &id}, None).await?;
            if let Some(user_doc) = updated_user_doc {
                let user = User {
                    id: user_doc.get_str("id").unwrap().to_string(),
                    name: user_doc.get_str("name").unwrap().to_string(),
                    email: user_doc.get_str("email").unwrap().to_string(),
                };
                Ok(user)
            } else {
                Err("User not found after update".into())
            }
        } else {
            Err("User not found".into())
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("users");

        let delete_result = collection.delete_one(doc! {"id": &id}, None).await?;
        Ok(delete_result.deleted_count > 0)
    }
}

#[derive(Default)]
pub struct Subgraph1Query;

#[Object]
impl Subgraph1Query {
    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("users");
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
