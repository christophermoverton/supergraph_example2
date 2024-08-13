use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix_web::{guard, HttpResponse};

mod schema;
mod db;

use db::connect_to_mongodb;

async fn graphql_handler(schema: web::Data<schema::AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mongodb = connect_to_mongodb("mongodb://localhost:27017", "test").await;

    let schema = schema::create_schema()
        .data(mongodb)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
