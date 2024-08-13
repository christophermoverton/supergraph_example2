use async_graphql::{Schema, SchemaBuilder, MergedObject};
use crate::schema::subgraph1::Subgraph1Query;
use crate::schema::subgraph2::Subgraph2Query;

pub mod subgraph1;
pub mod subgraph2;

#[derive(MergedObject, Default)]
pub struct QueryRoot(Subgraph1Query, Subgraph2Query);

pub type AppSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> SchemaBuilder<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription> {
    Schema::build(QueryRoot::default(), async_graphql::EmptyMutation, async_graphql::EmptySubscription)
}
