use mongodb::{Client, Database};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type MongoDb = Arc<Mutex<Database>>;

pub async fn connect_to_mongodb(uri: &str, db_name: &str) -> MongoDb {
    let client = Client::with_uri_str(uri).await.expect("Failed to initialize MongoDB client");
    let db = client.database(db_name);
    Arc::new(Mutex::new(db))
}
