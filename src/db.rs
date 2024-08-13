use neo4rs::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Neo4jGraph = Arc<Mutex<Graph>>;

pub async fn connect_to_neo4j(uri: &str, user: &str, password: &str) -> Neo4jGraph {
    let config = ConfigBuilder::default()
        .uri(uri) // Use the provided URI
        .user(user)
        .password(password)
        .db("neo4j")
        .fetch_size(500)
        .max_connections(10)
        .build()
        .unwrap();

    let graph = Graph::connect(config).await.unwrap();
    Arc::new(Mutex::new(graph))
}
