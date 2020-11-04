mod config;
mod graphql;
mod models;

use crate::config::APP_CONFIG;
use actix_web::{
    guard, middleware,
    web::{self, post},
    App, HttpServer,
};
use async_graphql::{extensions::ApolloTracing, EmptyMutation, Schema};
use graphql::{index, index_ws, Query, Subscription, SubscriptionServiceSchema};
use redis_async::{client, client::PubsubConnection};

async fn init_redis() -> PubsubConnection {
    let addr = format!("{}:{}", APP_CONFIG.redis.host, APP_CONFIG.redis.port)
        .parse()
        .expect("Cannot parse Redis connection string");

    client::pubsub_connect(&addr)
        .await
        .expect("Cannot connect to Redis")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_connection = init_redis().await;

    let schema: SubscriptionServiceSchema = Schema::build(Query, EmptyMutation, Subscription)
        .data(redis_connection)
        .extension(ApolloTracing)
        .finish();

    HttpServer::new(move || {
        App::new()
            // share GraphQL Schema
            .data(schema.clone())
            // enable logger
            .wrap(middleware::Logger::default())
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
