use crate::models::{AddRolesForUser, AddPermissionToRole, PermissionQuery};
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

    let authorized = e.has_permission_for_user(sub, vec![obj.to_owned(), act.to_owned()]);

    if authorized {
        Ok(HttpResponse::Ok().body("Is authorized"))
    } else {
        Ok(HttpResponse::Unauthorized().body("Not authorized"))
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

// #[post("/add-roles-for-user")]
/// Add user to roles
///
/// Basic Auth protected route
/// Adds a user (user id) to the given roles array
#[api_v2_operation]
pub async fn add_roles_for_user(
    enforcer: Data<Mutex<Enforcer>>,
    add_user: Json<AddRolesForUser>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let mut e = enforcer.lock().unwrap();

    let added = e
        .add_roles_for_user(&add_user.user_id, add_user.roles.clone(), Some(add_user.domain.as_str()))
        .await
        .expect("Cannot add policy");

    Ok(HttpResponse::Ok().body(format!("Added: {:?}", added)))
}

// #[post("/add-permissions-for-role")]
/// Add permissions to role
///
/// Basic Auth protected route
/// Adds all the permissions in the array to a role
#[api_v2_operation]
pub async fn add_permissions_for_role(
    enforcer: Data<Mutex<Enforcer>>,
    add_permission: Json<AddPermissionToRole>,
) -> std::result::Result<HttpResponse, HttpResponse> {
    let mut e = enforcer.lock().unwrap();

    let permissions: Vec<Vec<String>> = 
        add_permission
        .permissions
        .iter()
        .map(|permission| vec![permission.action.clone(), permission.object.clone()])
        .collect();

    let added = e
        .add_permissions_for_user(&add_permission.role, permissions)
        .await
        .expect("Cannot add policy");

    Ok(HttpResponse::Ok().body(format!("Added: {:?}", added)))
}
