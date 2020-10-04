mod authentication;
// mod authorization;
mod graphql;
mod models;

use actix_redis::RedisSession;
use actix_web::{cookie, middleware, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use authentication::routes::*;
use graphql::{index, IdentityServiceSchema, Query};
use models::User;
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use wither::mongodb::Client;
use wither::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect & sync indexes.
    let identity_database = Client::with_uri_str("mongodb://root:example@127.0.0.1:27017/")
        .await
        .expect("Cannot connect to the db")
        .database("identity-service");

    User::sync(&identity_database)
        .await
        .expect("Failed syncing indexes");

    let graphql_schema: IdentityServiceSchema =
        Schema::build(Query, EmptyMutation, EmptySubscription)
        // .extension(|| ApolloTracing::default())
        .data(identity_database.clone())
        .finish();

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
            .route("/graphql", actix_web::web::post().to(index))
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
