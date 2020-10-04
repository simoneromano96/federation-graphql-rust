#![feature(associated_type_bounds)]

mod authorization;
mod graphql;
mod models;

// use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use actix_redis::RedisSession;
use actix_web::{
    cookie, guard, middleware,
    web::{self, post},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
// use actix_cors::Cors;
// use actix_web_actors::ws;
use async_graphql::{
    extensions::ApolloTracing,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::WSSubscription;
// use std::sync::Arc;
use graphql::{index, Mutation, Query};
use models::Coffee;
use wither::prelude::*;

async fn init() -> wither::mongodb::Database {
    use wither::mongodb::Client;

    // Connect to the database.
    let products_database = Client::with_uri_str("mongodb://root:example@localhost:27017/admin")
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
            // enable logger
            .wrap(middleware::Logger::default())
            // cookie session middleware
            .wrap(
                RedisSession::new("127.0.0.1:6379", b"N7WoK3mG7lSb0CpK8UhAabUZNi27n5ub")
                    // Don't allow the cookie to be accessed from javascript
                    .cookie_http_only(true)
                    // allow the cookie only from the current domain
                    .cookie_same_site(cookie::SameSite::Lax),
            )
            // .wrap(Cors::default())
            .route("/graphql", post().to(index))
        /*
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
