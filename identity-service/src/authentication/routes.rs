use crate::authentication;
use crate::models::{User, UserInfo, UserInput};
use actix_session::Session;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, HttpResponse, Json},
};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Database as MongoDatabase;
use wither::prelude::*;

// #[post("/signup")]
/// User signup
///
/// Creates a new user but doesn't log in the user
/// Currently like this because of future developements
#[api_v2_operation]
pub async fn signup(
    db: Data<MongoDatabase>,
    new_user: Json<UserInput>,
) -> Result<Json<UserInfo>, HttpResponse> {
    let username = &new_user.username;
    let clear_password = &new_user.password;

    let password = authentication::hash_password(clear_password);

    // Create a user.
    let mut user = User::new_user(username, &password);

    if let Ok(_) = user.save(&db, None).await {
        Ok(Json(user.to_user_info()))
    } else {
        Err(HttpResponse::BadRequest().body("Username is already registered"))
    }
}

// #[post("/login")]
/// User login
///
/// Logs in the user via the provided credentials, will set a cookie session
#[api_v2_operation]
pub async fn login(
    credentials: Json<UserInput>,
    session: Session,
    db: Data<MongoDatabase>,
) -> Result<Json<UserInfo>, HttpResponse> {
    let maybe_user: Option<ObjectId> = session.get("user_id").unwrap();
    if let Some(user_id) = maybe_user {
        // We can be sure that the user exists if there is a session
        let user = User::find_by_id(&db, &user_id).await.unwrap();
        session.renew();
        Ok(Json(user.to_user_info()))
    } else {
        // let find_user_result: Result<Option<User>, wither::WitherError> =
        //     User::find_one(&db, doc! { "username": &credentials.username }, None).await;
        // if let Ok(find_user) = find_user_result {
        if let Some(user) = User::find_by_username(&db, &credentials.username).await {
            let clear_password = &credentials.password;
            let hashed_password = &user.password;

            let password_verified = authentication::verify_hash(hashed_password, clear_password);

            if password_verified {
                let info = user.to_user_info();
                // If the user exists there is a user id
                session.set("user_id", user.id.unwrap()).unwrap();
                session.set("user_role", user.role).unwrap();
                Ok(Json(info))
            } else {
                Err(HttpResponse::BadRequest().body("Wrong password"))
            }
        } else {
            Err(HttpResponse::NotFound().body("User not found"))
        }
        // } else {
        //     Err(HttpResponse::InternalServerError().body(""))
        // }
    }
}

// #[get("/user-info")]
/// User info
///
/// Gets the current user info if he is logged in
#[api_v2_operation]
pub async fn user_info(
    session: Session,
    db: Data<MongoDatabase>,
) -> Result<Json<UserInfo>, HttpResponse> {
    let maybe_id: Option<ObjectId> = session.get("user_id").unwrap();

    if let Some(id) = maybe_id {
        let maybe_user = User::find_by_id(&db, &id).await;
        if let Some(user) = maybe_user {
            session.renew();
            Ok(Json(user.to_user_info()))
        } else {
            session.clear();
            Err(HttpResponse::Unauthorized().body("Error"))
        }
    } else {
        Err(HttpResponse::Unauthorized().body("Not logged in"))
    }
}

// #[get("/logout")]
/// Logout
///
/// Logs out the current user invalidating the session if he is logged in
#[api_v2_operation]
pub async fn logout(session: Session) -> Result<HttpResponse, HttpResponse> {
    let maybe_user: Option<ObjectId> = session.get("user_id").unwrap();

    if let Some(_) = maybe_user {
        session.clear();
        Ok(HttpResponse::Ok().body("Logged out"))
    } else {
        Err(HttpResponse::BadRequest().body("Already logged out"))
    }
}
