mod authentication;
mod graphql;
mod models;

use actix_redis::RedisSession;
use actix_web::{cookie, middleware, App, HttpServer};
use async_graphql::{
    extensions::apollo_persisted_queries::ApolloPersistedQueries,
    extensions::apollo_persisted_queries::LruCacheStorage, extensions::ApolloTracing,
    EmptyMutation, EmptySubscription, Schema,
};
use authentication::routes::*;
use graphql::{gql_playgound, index, IdentityServiceSchema, Query};
use models::User;
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use wither::mongodb::{Client, Database};
use wither::Model;

async fn init_db() -> Database {
    Client::with_uri_str("mongodb://root:example@127.0.0.1:27017/")
        .await
        .expect("Cannot connect to the db")
        .database("identity-service")
}

fn init_graphql(db: &Database) -> IdentityServiceSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db.clone())
        .extension(ApolloTracing)
        .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect & sync indexes.
    let identity_database = init_db().await;

    User::sync(&identity_database)
        .await
        .expect("Failed syncing indexes");

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
                RedisSession::new("127.0.0.1:6379", b"N7WoK3mG7lSb0CpK8UhAabUZNi27n5ub")
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
    .bind("0.0.0.0:4001")?
    .run()
    .await
}
