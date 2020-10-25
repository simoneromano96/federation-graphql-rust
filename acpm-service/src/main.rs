mod middlewares;
mod models;
mod routes;

use actix_web::{web::Data, App, HttpServer};
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use routes::{add_policy, is_authorized, remove_policy};
use sqlx_adapter::{casbin::prelude::*, casbin::Result, SqlxAdapter};
use std::sync::Mutex;

async fn init_db() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
}

async fn init_casbin() -> Result<Enforcer> {
    let m = DefaultModel::from_file("C:\\Users\\Sippo\\Desktop\\git\\github\\federation-graphql-rust\\acpm-service\\src\\access_model\\rbac_model.conf").await?;
    let a = SqlxAdapter::new("postgres://casbin:casbin@127.0.0.1:5432/casbin", 8).await?;
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

    HttpServer::new(move || {
        App::new()
            .app_data(enforcer.clone())
            .wrap_api()
            .service(
                scope("/api").service(
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
    .bind("0.0.0.0:4001")?
    .run()
    .await
}
