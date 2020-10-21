#![feature(associated_type_bounds)]
#![feature(async_closure)]

mod authorization;
mod graphql;
mod models;
mod utils;

// use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use actix_redis::RedisSession;
use actix_web::{
    cookie, guard, middleware,
    web::{self, post},
    App, HttpServer,
};
// use actix_cors::Cors;
// use actix_web_actors::ws;
use async_graphql::{extensions::ApolloTracing, EmptySubscription, Schema};
// use async_graphql_actix_web::WSSubscription;
// use std::sync::Arc;
use graphql::{index, index_ws, Mutation, ProductsServiceSchema, Query};
use models::Coffee;
// use redis_async::client::pubsub_connect;
use wither::prelude::*;

/*
pub struct AppData {
    mongo_database: wither::mongodb::Database,
    redis_publish: redis::aio::Connection,
    // redis_pubsub: redis::aio::PubSub,
}
*/

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

async fn init_redis() -> redis::aio::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Cannot connect redis client");

    let redis_connection: redis::aio::Connection = client
        .get_async_connection()
        .await
        .expect("Cannot get redis connection");
    // let pubsub_conn: redis::aio::PubSub = client.get_async_connection().await.expect("Cannot get redis connection").into_pubsub();

    redis_connection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init().await;

    let mut redis_connection: redis::aio::Connection = init_redis().await;


    let schema: ProductsServiceSchema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .data(redis_connection)
        // .extension(ApolloTracing)
        .finish();

    
    HttpServer::new(move || {
        App::new()
            // share GraphQL Schema
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
            // CORS
            // .wrap(Cors::default())
            // GraphQL
            .route("/graphql", post().to(index))
            // GraphQL Subscriptions
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
        /*
        .service(web::resource("/playground").guard(guard::Get()).to(index_playground))
        */
    })
    .bind("0.0.0.0:4002")?
    .run()
    .await
}
