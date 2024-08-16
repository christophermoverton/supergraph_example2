use actix_web::{web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
//use async_graphql::{Schema, MergedObject};
//use redis::AsyncCommands;
use redis::Client;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;

async fn graphql_handler(
    schema: web::Data<schema::AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = Client::open(redis_url).expect("Invalid Redis URL");
    let schema = schema::create_schema(redis_client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(actix_web::guard::Post())
                    .to(graphql_handler),
            )
            .service(
                web::resource("/playground")
                    .guard(actix_web::guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
