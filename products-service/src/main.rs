#![feature(associated_type_bounds)]

mod graphql;
mod models;

// use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, guard, web::{self, post}};
// use actix_cors::Cors;
// use actix_web_actors::ws;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, extensions::ApolloTracing, http::{playground_source, GraphQLPlaygroundConfig}};
use async_graphql_actix_web::{WSSubscription};
// use std::sync::Arc;
use graphql::{Mutation, Query, index};
use models::Coffee;
use wither::prelude::*;

/*
async fn index(schema: web::Data<CoffeeSchema>, req: GQLRequest) -> GQLResponse {
    let inner = req.into_inner();
    // let inner: QueryBuilder = req.into_inner();
    // inner.execute(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

async fn index_ws(
    schema: web::Data<CoffeeSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}
*/

async fn init() -> wither::mongodb::Database {
    use wither::mongodb::Client;

    // Connect to the database.
    let products_database =
        Client::with_uri_str("mongodb://root:example@localhost:27017/admin")
            .await
            .expect("Cannot connect to the db")
            .database("products-service");

    Coffee::sync(&products_database)
        .await
        .expect("Failed syncing indexes");

    products_database
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        // .extension(|| ApolloTracing::default())
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            // .wrap(Cors::default())
            .route("/graphql", post().to(index))
        /*
        .service(web::resource("/graphql").guard(guard::Post()).to(index))
        .service(
            web::resource("/graphql")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(web::resource("/playground").guard(guard::Get()).to(index_playground))
        */
    })
    .bind("0.0.0.0:4002")?
    .run()
    .await
}
