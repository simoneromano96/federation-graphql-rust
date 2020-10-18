use std::sync::{Arc, Mutex};

use paperclip::actix::{
    api_v2_operation,
    web::{Data, HttpResponse, Json, Query},
};
use sqlx_adapter::casbin::prelude::*;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Database as MongoDatabase;

use crate::models::{PermissionQuery, Role};
use crate::models::{User, UserInfo, UserInput};

// #[get("/is-authorized")]
/// Is authorized
///
/// Should be a Basic Auth protected route
/// Gives back if a user (subject) can do something (action) to something else (object)
#[api_v2_operation]
pub async fn is_authorized(
    db: Data<MongoDatabase>,
    enforcer: Data<Mutex<Enforcer>>,
    permission_query: Query<PermissionQuery>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let sub = ObjectId::with_string(&permission_query.subject).unwrap();
    // let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;
    let e = enforcer.lock().unwrap(); 

    if let Some(user) = User::find_by_id(&db, &sub).await {
        let r = e.enforce((&user.role, obj, act));
        if let Ok(authorized) = r {
            if authorized {
                Ok(HttpResponse::Ok().body("Is authorized"))
            } else {
                Ok(HttpResponse::Unauthorized().body("Not authorized"))
            }
        } else {
            Err(HttpResponse::InternalServerError().body("Oopsie woopsie!"))
        }
    } else {
        Err(HttpResponse::BadRequest().body("Cannot find user"))
    }
}

// #[get("/add-policy")]
/// Add an access policy
///
/// Should be a Basic Auth protected route
///
#[api_v2_operation]
pub async fn add_policy(
    db: Data<MongoDatabase>,
    enforcer: Data<Mutex<Enforcer>>,
    permission_query: Json<PermissionQuery>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;
    let mut e = enforcer.lock().unwrap(); 

    let added = e
        .add_named_policy(
            "p",
            vec![sub.clone(), obj.clone(), act.clone()],
        )
        .await.expect("Cannot add policy");

    Ok(HttpResponse::Ok().body(format!("Added: {:?}", added)))
}
