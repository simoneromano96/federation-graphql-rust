mod authorization;
mod config;
mod graphql;
mod models;
mod utils;

// use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use std::net::SocketAddr;

use actix_redis::RedisSession;
use actix_web::{
    cookie, guard, middleware,
    web::{self, post},
    App, HttpServer,
};
// use actix_cors::Cors;
// use actix_web_actors::ws;
use async_graphql::{extensions::ApolloTracing, EmptySubscription, Schema};
use base64;
// use async_graphql_actix_web::WSSubscription;
// use std::sync::Arc;
use graphql::{
    gql_playgound, index, index_ws, Mutation, ProductsServiceSchema, Query, Subscription,
};
use models::Coffee;
// use redis_async::client::pubsub_connect;
use crate::config::APP_CONFIG;
use pretty_env_logger;
use redis_async::{
    client, client::paired::PairedConnection, client::PubsubConnection, resp::FromResp,
};
use reqwest::{header, Client, ClientBuilder};
use wither::prelude::*;

/*
pub struct AppData {
    mongo_database: wither::mongodb::Database,
    redis_publish: redis::aio::Connection,
    // redis_pubsub: redis::aio::PubSub,
}
*/

async fn init_mongo() -> wither::mongodb::Database {
    use wither::mongodb::Client;

    // Connect to the database.
    let products_database = Client::with_uri_str(&APP_CONFIG.mongo.connection_string)
        .await
        .expect("Cannot connect to the db")
        .database("products-service");

    Coffee::sync(&products_database)
        .await
        .expect("Failed syncing indexes");

    products_database
}

async fn init_redis() -> (PairedConnection, PubsubConnection) {
    let addr = format!("{}:{}", APP_CONFIG.redis.host, APP_CONFIG.redis.port)
        .parse()
        .expect("Cannot parse Redis connection string");

    (
        client::paired_connect(&addr)
            .await
            .expect("Cannot open connection"),
        client::pubsub_connect(&addr)
            .await
            .expect("Cannot connect to Redis"),
    )

    /*
    let client = redis::Client::open("redis://127.0.0.1/").expect("Cannot connect redis client");

    let redis_connection: redis::aio::Connection = client
        .get_async_connection()
        .await
        .expect("Cannot get redis connection");
    // let pubsub_conn: redis::aio::PubSub = client.get_async_connection().await.expect("Cannot get redis connection").into_pubsub();

    redis_connection
    */
}

fn init_http_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    let basic_decoded = format!(
        "Basic {}:{}",
        APP_CONFIG.authorization_server.basic_auth.username,
        APP_CONFIG.authorization_server.basic_auth.password
    );
    let basic_auth_header_value = base64::encode(basic_decoded);

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&basic_auth_header_value).expect("Invalid header value"),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Could not create http client")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let db = init_mongo().await;
    let redis_connection = init_redis().await;
    let http_client = init_http_client();

    let schema: ProductsServiceSchema = Schema::build(Query, Mutation, Subscription)
        .data(db)
        .data(redis_connection)
        .data(http_client)
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
                RedisSession::new(
                    format!("{:?}:{:?}", APP_CONFIG.redis.host, APP_CONFIG.redis.port),
                    APP_CONFIG.session.secret.as_bytes(),
                )
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
        // .service(web::resource("/playground").guard(guard::Get()).to(gql_playgound))
    })
    .bind(format!("0.0.0.0:{:?}", APP_CONFIG.server.port))?
    .run()
    .await
}
