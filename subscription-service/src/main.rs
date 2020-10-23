#![feature(associated_type_bounds)]
#![feature(async_closure)]

mod graphql;
mod models;

// use crate::graphql::coffee::{CoffeeSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use actix_web::{
    guard, middleware,
    web::{self, post},
    App, HttpServer,
};
// use actix_cors::Cors;
// use actix_web_actors::ws;
use async_graphql::{extensions::ApolloTracing, EmptyMutation, Schema};
// use async_graphql_actix_web::WSSubscription;
// use std::sync::Arc;
use graphql::{index, index_ws, Query, Subscription, SubscriptionServiceSchema};
// use redis_async::client::pubsub_connect;
use redis_async::{client, client::PubsubConnection};

async fn init_redis() -> PubsubConnection {
    let addr = "127.0.0.1:6379"
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
        // .extension(ApolloTracing)
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
    .bind("0.0.0.0:4003")?
    .run()
    .await
}
