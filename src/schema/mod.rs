use async_graphql::{Schema, MergedObject};
use crate::schema::subgraph1::{Subgraph1Query, Subgraph1Mutation};
use crate::schema::subgraph2::{Subgraph2Query, Subgraph2Mutation};
use sqlx::MySqlPool;

pub mod subgraph1;
pub mod subgraph2;
pub mod scalars;  // Include scalars.rs

#[derive(MergedObject, Default)]
pub struct QueryRoot(Subgraph1Query, Subgraph2Query);

#[derive(MergedObject, Default)]
pub struct MutationRoot(Subgraph1Mutation, Subgraph2Mutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

pub fn create_schema(pool: MySqlPool) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), async_graphql::EmptySubscription)
        .data(pool)
        .finish()
}
