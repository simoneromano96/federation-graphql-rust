use std::sync::{Arc, Mutex};
use actix_session::Session;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, HttpResponse, Json, Query},
};
use sqlx_adapter::casbin::{CoreApi, Enforcer};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Database as MongoDatabase;
use wither::prelude::*;

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
    enforcer: Data<Arc<Enforcer>>,
    permission_query: Query<PermissionQuery>,
) -> Result<HttpResponse, HttpResponse> {
    let sub = &permission_query.subject;
    let obj = &permission_query.object;
    let act = &permission_query.action;

    let e = enforcer;
    
    let r = e.enforce(&[sub, obj, act]);

    /*
    if let Some(user) = User::find_by_id(&db, &permission_query.subject).await {

        // TODO
        match user.role {
            Role::Admin => {
                Ok(HttpResponse::Ok().body(""))
            }
            _ => {
                Err(HttpResponse::Forbidden().body("Does not have permission"))
            }
        }
    } else {
        Err(HttpResponse::BadRequest().body("Cannot find user"))
    }
    */
}

