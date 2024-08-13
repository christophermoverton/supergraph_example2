use async_graphql::{Context, Object, Result, SimpleObject, InputObject};
use crate::db::MongoDb;
use mongodb::bson::{doc, Bson};
use mongodb::options::UpdateOptions;

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
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("products");

        let product_doc = doc! {
            "id": &input.id,
            "name": &input.name,
            "price": input.price,
        };

        collection.insert_one(product_doc.clone(), None).await?;
        let product = Product {
            id: input.id,
            name: input.name,
            price: input.price,
        };
        Ok(product)
    }

    async fn update_product(&self, ctx: &Context<'_>, id: String, name: Option<String>, price: Option<f64>) -> Result<Product> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("products");

        let mut update_doc = doc! {};
        if let Some(name) = name {
            update_doc.insert("name", Bson::String(name));
        }
        if let Some(price) = price {
            update_doc.insert("price", Bson::Double(price));
        }

        let update_result = collection.update_one(doc! {"id": &id}, doc! {"$set": update_doc}, UpdateOptions::builder().upsert(true).build()).await?;
        
        if update_result.matched_count > 0 {
            let updated_product_doc = collection.find_one(doc! {"id": &id}, None).await?;
            if let Some(product_doc) = updated_product_doc {
                let product = Product {
                    id: product_doc.get_str("id").unwrap().to_string(),
                    name: product_doc.get_str("name").unwrap().to_string(),
                    price: product_doc.get_f64("price").unwrap(),
                };
                Ok(product)
            } else {
                Err("Product not found after update".into())
            }
        } else {
            Err("Product not found".into())
        }
    }

    async fn delete_product(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("products");

        let delete_result = collection.delete_one(doc! {"id": &id}, None).await?;
        Ok(delete_result.deleted_count > 0)
    }
}

#[derive(Default)]
pub struct Subgraph2Query;

#[Object]
impl Subgraph2Query {
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {
        let db = ctx.data::<MongoDb>()?.lock().await;
        let collection: mongodb::Collection<mongodb::bson::Document> = db.collection("products");

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
