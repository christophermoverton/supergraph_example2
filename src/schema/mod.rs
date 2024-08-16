// src/schema/mod.rs

use async_graphql::{Schema, MergedObject};
use redis::Client;

pub mod subgraph1;
pub mod subgraph2;

// QueryRoot combines all queries from subgraphs
#[derive(MergedObject, Default)]
pub struct QueryRoot(subgraph1::UserQuery, subgraph2::ProductQuery);

// MutationRoot combines all mutations from subgraphs
#[derive(MergedObject, Default)]
pub struct MutationRoot(subgraph1::UserMutation, subgraph2::ProductMutation);

// Type alias for the entire GraphQL schema
pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

// Function to create and return the GraphQL schema, injecting the Redis client
pub fn create_schema(redis_client: Client) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), async_graphql::EmptySubscription)
        .data(redis_client)
        .finish()
}
