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
use async_graphql::{Schema, extensions::ApolloTracing, extensions::{Logger, apollo_persisted_queries::{ApolloPersistedQueries, LruCacheStorage}}};
use base64;
use graphql::{index, index_ws, Mutation, ProductsServiceSchema, Query, Subscription};
use log::info;
use models::Coffee;
use pretty_env_logger;
// use redis_async::client::{paired::PairedConnection, PubsubConnection};
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

    info!("Mongo database initialised");

    Coffee::sync(&products_database)
        .await
        .expect("Failed syncing indexes");

    info!("Models synced");

    products_database
}

async fn init_redis() -> redis::Client {
    // use redis_async::client;

    /*
    let addr = format!("{}:{}", APP_CONFIG.redis.host, APP_CONFIG.redis.port)
        .parse()
        .expect("Cannot parse Redis connection string");
    */
    let addr = format!("redis://{}:{}", APP_CONFIG.redis.host, APP_CONFIG.redis.port);

    let client: redis::Client = redis::Client::open(addr).unwrap();

    info!("Redis client initialised");

    client
    /*
    (
        client::paired_connect(&addr)
            .await
            .expect("Cannot open connection"),
        client::pubsub_connect(&addr)
            .await
            .expect("Cannot connect to Redis"),
    )
    */
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

    info!("HTTP Client initialised");

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Could not create http client")
}

fn init_graphql(
    db: wither::mongodb::Database,
    redis_client: redis::Client,
    http_client: reqwest::Client,
) -> ProductsServiceSchema {
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(db)
        .data(redis_client)
        .data(http_client)
        // .extension(ApolloTracing)
        // .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .extension(Logger)
        .finish();
    
    info!("Initialised graphql");
    
    schema
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("called main()");
    if APP_CONFIG.debug {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LOG", "info,actix_web=info,actix_redis=info");
    }

    pretty_env_logger::init();

    let db = init_mongo().await;
    let redis_client = init_redis().await;
    let http_client = init_http_client();
    let schema = init_graphql(db, redis_client, http_client);
    info!("Initialisation finished, server will listen at port: {:?}", APP_CONFIG.server.port);

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
