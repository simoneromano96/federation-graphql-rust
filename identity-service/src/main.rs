mod authentication;
mod config;
mod graphql;
mod models;

use crate::config::APP_CONFIG;
use actix_redis::RedisSession;
use actix_web::{cookie, middleware, App, HttpServer};
use async_graphql::{
    extensions::{apollo_persisted_queries::ApolloPersistedQueries, ApolloTracing, Logger, apollo_persisted_queries::LruCacheStorage},
    EmptyMutation, EmptySubscription, Schema,
};
use authentication::routes::*;
use graphql::{gql_playgound, index, IdentityServiceSchema, Query};
use models::User;
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use log::info;
use wither::mongodb::{Client, Database};
use wither::Model;

async fn init_db() -> Database {
    let db = Client::with_uri_str(&APP_CONFIG.database.url)
        .await
        .expect("Cannot connect to the db")
        .database("identity-service");
    
    info!("Mongo database initialised");

    User::sync(&db)
        .await
        .expect("Failed syncing indexes");

    db
}

fn init_graphql(db: &Database) -> IdentityServiceSchema {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db.clone())
        // .extension(ApolloTracing)
        // .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .extension(Logger)
        .finish();
    
    info!("Initialised graphql");

    schema
}

fn init_logger() {
    if APP_CONFIG.debug {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LOG", "info,actix_web=info,actix_redis=info");
    }

    pretty_env_logger::init();
    info!("Logger initialised");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("called main()");
    
    init_logger();

    // Connect & sync indexes.
    let identity_database = init_db().await;
    let graphql_schema = init_graphql(&identity_database);

    // let db = std::sync::Arc::new(identity_database);

    // std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
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
            .data(identity_database.clone())
            .data(graphql_schema.clone())
            // GraphQL
            .route("/graphql", actix_web::web::post().to(index))
            .route("/playground", actix_web::web::get().to(gql_playgound))
            // Record services and routes from this line.
            .wrap_api()
            .service(
                scope("/api")
                .service(
                    scope("/v1")
                        .route("/signup", post().to(signup))
                        .route("/login", post().to(login))
                        .route("/user-info", get().to(user_info))
                        .route("/logout", get().to(logout))
                        // .service(signup)
                        // .service(login)
                        // .service(user_info)
                        // .service(logout),
                ),
            )
            // Mount the JSON spec at this path.
            .with_json_spec_at("/openapi")
            // Build the app
            .build()
    })
    .bind(format!("0.0.0.0:{:?}", APP_CONFIG.server.port))?
    .run()
    .await
}
