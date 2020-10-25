mod config;
mod middlewares;
mod models;
mod routes;

use crate::config::APP_CONFIG;
use actix_web::{web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use middlewares::basic_auth_validator;
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use routes::{add_policy, is_authorized, remove_policy};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx_adapter::{
    casbin::{self, prelude::*},
    SqlxAdapter,
};
use std::sync::{Arc, Mutex};

async fn init_db() -> sqlx::Result<Pool<Postgres>> {
    // Create a connection pool
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(APP_CONFIG.database.pool_size)
        .connect(&APP_CONFIG.database.url)
        .await?;

    Ok(pool)
}

async fn init_casbin() -> casbin::Result<Enforcer> {
    let m = DefaultModel::from_file(&APP_CONFIG.access_model_path).await?;
    let a = SqlxAdapter::new(&APP_CONFIG.database.url, APP_CONFIG.database.pool_size).await?;
    let e = Enforcer::new(m, a).await?;

    Ok(e)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let enforcer = Data::new(Mutex::new(
        init_casbin()
            .await
            .expect("could not create access policy enforcer"),
    ));

    let pool = init_db().await.expect("Could not init db");

    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(basic_auth_validator);

        App::new()
            .app_data(enforcer.clone())
            .app_data(pool.clone())
            // .wrap(auth)
            .wrap_api()
            .service(
                scope("/api")
                    // Protect the following routes with Basic Auth
                    .wrap(auth)
                    .service(
                        scope("/v1").service(
                            scope("/authorization")
                                .route("/is-authorized", get().to(is_authorized))
                                .route("/add-policy", post().to(add_policy))
                                .route("/remove-policy", post().to(remove_policy)),
                        ),
                    ),
            )
            // Mount the JSON spec at this path.
            .with_json_spec_at("/openapi")
            // Build the app
            .build()
    })
    .bind(format!("0.0.0.0:{}", APP_CONFIG.server.port))?
    .run()
    .await
}
