use async_graphql::{Object};

#[derive(Default)]
pub struct Subgraph2Query;

#[Object]
impl Subgraph2Query {
    async fn world(&self) -> &str {
        "Hello from Subgraph 2!"
    }
}

#[derive(Default)]
pub struct Subgraph2Mutation;

#[Object]
impl Subgraph2Mutation {
    async fn concatenate(&self, str1: String, str2: String) -> String {
        format!("{}{}", str1, str2)
    }
}
