mod config;
mod middlewares;
mod models;
mod routes;

use crate::config::APP_CONFIG;
use actix_web::{App, HttpServer, web::Data, middleware};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::info;
use middlewares::basic_auth_validator;
use paperclip::actix::{
    web::{get, post, scope},
    OpenApiExt,
};
use routes::{add_policy, add_user_to_role, is_authorized, remove_policy};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx_adapter::{
    casbin::{self, prelude::*},
    SqlxAdapter,
};
use std::sync::{Arc, Mutex};

async fn init_db() -> sqlx::Result<Pool<Postgres>> {
    // Create a connection pool
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(APP_CONFIG.database.poolsize)
        .connect(&APP_CONFIG.database.url)
        .await?;

    info!("DB client initialised");

    Ok(pool)
}

async fn init_casbin() -> casbin::Result<Enforcer> {
    let m = DefaultModel::from_file(&APP_CONFIG.accessmodelpath).await?;
    let a = SqlxAdapter::new(&APP_CONFIG.database.url, APP_CONFIG.database.poolsize).await?;
    let e = Enforcer::new(m, a).await?;

    info!("Casbin initialised");
    Ok(e)
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

    let enforcer = Data::new(Mutex::new(
        init_casbin()
            .await
            .expect("could not create access policy enforcer"),
    ));

    let pool = init_db().await.expect("Could not init db");
    info!("Initialisation finished, server will listen at port: {:?}", APP_CONFIG.server.port);

    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(basic_auth_validator);

        App::new()
            .app_data(enforcer.clone())
            .app_data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
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
                                .route("/remove-policy", post().to(remove_policy))
                                .route("/add-user-to-role", post().to(add_user_to_role)),
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
