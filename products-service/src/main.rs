mod authorization;
mod config;
mod graphql;
mod models;

use crate::config::APP_CONFIG;
use actix_redis::RedisSession;
use actix_web::{
    cookie, guard, middleware,
    web::{self, post},
    App, HttpServer,
};
use async_graphql::{
    extensions::apollo_persisted_queries::{ApolloPersistedQueries, LruCacheStorage},
    extensions::ApolloTracing,
    EmptySubscription, Schema,
};
use base64;
use graphql::{index, index_ws, Mutation, ProductsServiceSchema, Query};
use models::Coffee;
use pretty_env_logger;
use redis_async::client::paired::PairedConnection;
use reqwest::{header, ClientBuilder};
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
    let products_database = Client::with_uri_str(&APP_CONFIG.database.url)
        .await
        .expect("Cannot connect to the db")
        .database("products-service");

    Coffee::sync(&products_database)
        .await
        .expect("Failed syncing indexes");

    products_database
}

async fn init_redis() -> PairedConnection {
    use redis_async::client;

    let addr = format!("{}:{}", APP_CONFIG.redis.host, APP_CONFIG.redis.port)
        .parse()
        .expect("Cannot parse Redis connection string");

    client::paired_connect(&addr)
        .await
        .expect("Cannot open connection")
}

fn init_http_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    let basic_credentials = format!(
        "{}:{}",
        APP_CONFIG.authorization.auth.username,
        APP_CONFIG.authorization.auth.password
    );
    let basic_auth_header_value = format!("Basic {}", base64::encode(basic_credentials));

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&basic_auth_header_value).expect("Invalid header value"),
    );

    // println!("{:?}", headers);

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Could not create http client")
}

fn init_graphql(
    db: wither::mongodb::Database,
    redis_connection: PairedConnection,
    http_client: reqwest::Client,
) -> ProductsServiceSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .data(redis_connection)
        .data(http_client)
        .extension(ApolloTracing)
        .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let db = init_mongo().await;
    let redis_connection = init_redis().await;
    let http_client = init_http_client();
    let schema = init_graphql(db, redis_connection, http_client);

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
                .cookie_same_site(cookie::SameSite::Strict),
            )
            // CORS
            // .wrap(Cors::default())
            // GraphQL
            .route("/graphql", post().to(index))
            // GraphQL Subscriptions
            // .service(
            //     web::resource("/graphql")
            //         .guard(guard::Get())
            //         .guard(guard::Header("upgrade", "websocket"))
            //         .to(index_ws),
            // )
        // .service(web::resource("/playground").guard(guard::Get()).to(gql_playgound))
    })
    .bind(format!("0.0.0.0:{:?}", APP_CONFIG.server.port))?
    .run()
    .await
}
