mod authentication;

use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{
    cookie, get, middleware, post,
    web::{self, scope},
    App, Error, HttpResponse, HttpServer, Responder,
};
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::{Client, Database as MongoDatabase};
use wither::prelude::*;

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
/// User representation
struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The username.
    username: String,
    /// The hashed password.
    password: String,
    // User email
    // email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// The username.
    username: String,
}

impl User {
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// New User Input
struct UserInput {
    /// The username.
    username: String,
    /// The "clear" password.
    password: String,
    // User email
    // email: String,
}

#[post("/signup")]
async fn signup(db: web::Data<MongoDatabase>, new_user: web::Json<UserInput>) -> HttpResponse {
    let username = &new_user.username;
    let clear_password = &new_user.password;

    let password = authentication::hash_password(clear_password);

    // Create a user.
    let mut user = User {
        id: None,
        username: username.clone(),
        password,
    };

    if let Ok(_) = user.save(&db, None).await {
        HttpResponse::Ok().json(user.to_user_info())
    } else {
        HttpResponse::BadRequest().body("Username is already registered")
    }
}

#[post("/login")]
async fn login(
    credentials: web::Json<UserInput>,
    session: Session,
    db: web::Data<MongoDatabase>,
) -> HttpResponse {
    let maybe_user: Option<User> = session.get("user").unwrap();
    if let Some(_) = maybe_user {
        session.renew();
        HttpResponse::Ok().body("Already logged in")
    } else {
        let find_user_result: Result<Option<User>, wither::WitherError> =
            User::find_one(&db, doc! { "username": &credentials.username }, None).await;

        if let Ok(find_user) = find_user_result {
            if let Some(user) = find_user {
                let clear_password = &credentials.password;
                let hashed_password = &user.password;

                let password_verified =
                    authentication::verify_hash(hashed_password, clear_password);

                if password_verified {
                    session.set("user", user).unwrap();
                    HttpResponse::Ok().body("Logged in")
                } else {
                    HttpResponse::BadRequest().body("Wrong password")
                }
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        } else {
            HttpResponse::InternalServerError().body("")
        }
    }
}

#[get("/user-info")]
async fn user_info(
    session: Session,
    // db: web::Data<MongoDatabase>,
) -> HttpResponse {
    let maybe_user: Option<User> = session.get("user").unwrap();

    if let Some(user) = maybe_user {
        session.renew();
        HttpResponse::Ok().json(user.to_user_info())
    } else {
        HttpResponse::Unauthorized().body("Not logged in")
    }
}

#[get("/logout")]
async fn logout(session: Session) -> HttpResponse {
    let maybe_user: Option<User> = session.get("user").unwrap();

    if let Some(_) = maybe_user {
        session.clear();
        HttpResponse::Ok().body("Logged out")
    } else {
        HttpResponse::BadRequest().body("Already logged out")
    }
}

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

    // let db = std::sync::Arc::new(identity_database);

    // std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(identity_database.clone())
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
            .service(
                scope("/api").service(
                    scope("/v1")
                        .service(signup)
                        .service(login)
                        .service(user_info)
                        .service(logout),
                ),
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
