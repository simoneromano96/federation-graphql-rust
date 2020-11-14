use crate::models::{AddUserToRole, PermissionQuery};
use paperclip::actix::{
    api_v2_operation,
    web::{Data, HttpResponse, Json, Query},
};
use sqlx_adapter::casbin::prelude::*;
use std::sync::Mutex;

// #[get("/is-authorized")]
/// Is authorized
///
/// Basic Auth protected route
/// Gives back if a user (subject) can do something (action) to something else (object)
#[api_v2_operation]
pub async fn is_authorized(
    enforcer: Data<Mutex<Enforcer>>,
    permission_query: Query<PermissionQuery>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;
    let e = enforcer.lock().unwrap();

    let r = e.enforce((&sub, obj, act));
    if let Ok(authorized) = r {
        if authorized {
            Ok(HttpResponse::Ok().body("Is authorized"))
        } else {
            Ok(HttpResponse::Unauthorized().body("Not authorized"))
        }
    } else {
        Err(HttpResponse::InternalServerError().body("Oopsie woopsie!"))
    }
}

// #[post("/add-policy")]
/// Add an access policy
///
/// Basic Auth protected route
/// Adds an access policy
#[api_v2_operation]
pub async fn add_policy(
    enforcer: Data<Mutex<Enforcer>>,
    permission_query: Json<PermissionQuery>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;
    let mut e = enforcer.lock().unwrap();

    let added = e
        .add_named_policy("p", vec![sub.clone(), obj.clone(), act.clone()])
        .await
        .expect("Cannot add policy");

    Ok(HttpResponse::Ok().body(format!("Added: {:?}", added)))
}

// #[post("/remove-policy")]
/// Remove an access policy
///
/// Basic Auth protected route
/// Removes an access policy
#[api_v2_operation]
pub async fn remove_policy(
    enforcer: Data<Mutex<Enforcer>>,
    permission_query: Json<PermissionQuery>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;
    let mut e = enforcer.lock().unwrap();

    let removed = e
        .remove_named_policy("p", vec![sub.clone(), obj.clone(), act.clone()])
        .await
        .expect("Cannot remove policy");

    Ok(HttpResponse::Ok().body(format!("Removed: {:?}", removed)))
}

// #[post("/add-user-to-role")]
/// Add user to role policy
///
/// Basic Auth protected route
/// Adds an access policy
#[api_v2_operation]
pub async fn add_user_to_role(
    enforcer: Data<Mutex<Enforcer>>,
    add_user: Json<AddUserToRole>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let mut e = enforcer.lock().unwrap();

    // TODO: add domain support
    let added = e
        .add_role_for_user(&add_user.user_id, &add_user.role, None)
        .await
        .expect("Cannot add policy");

    Ok(HttpResponse::Ok().body(format!("Added: {:?}", added)))
}
