use async_graphql::{Object};

#[derive(Default)]
pub struct Subgraph1Query;

#[Object]
impl Subgraph1Query {
    async fn hello(&self) -> &str {
        "Hello from Subgraph 1!"
    }
}

#[derive(Default)]
pub struct Subgraph1Mutation;

#[Object]
impl Subgraph1Mutation {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
