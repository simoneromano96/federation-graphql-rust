use async_graphql::{Context, Object, Result, ID};
use wither::{bson::doc, mongodb::Database};
use wither::Model;

use crate::models::{User, UserInfo};

pub struct Query;

async fn find_user_by_id(db: &Database, id: ID) -> Option<User> {
    let res = User::find_one(&db, doc! { "_id": id.to_string() }, None).await;
    if let Ok(maybe_user) = res {
        maybe_user
    } else {
        None
    }
}

/*
async fn find_user_by_username(db: &Database, username: String) -> Option<User> {
    let res = User::find_one(&db, doc! { "username": username }, None).await;
    if let Ok(maybe_user) = res {
        maybe_user
    } else {
        None
    }
}
*/

#[Object]
impl Query {
    /// Get current user info
    async fn me(&self, ctx: &Context<'_>) -> Result<UserInfo> {
        // let user = ctx.data::<User>();
        if let Ok(user) = ctx.data::<User>() {
            Ok(user.to_user_info())
        } else {
            Err("Not logged in".into())
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<UserInfo> {
        let db: &Database = ctx.data()?;
        let maybe_user = find_user_by_id(db, id).await;
        if let Some(user) = maybe_user {
            Ok(user.to_user_info())
        } else {
            Err("No user found".into())
        }
    }

    /*
    #[graphql(entity)]
    async fn find_user_by_username(&self, ctx: &Context<'_>, username: String) -> Result<UserInfo> {
        let db: &Database = ctx.data()?;
        let maybe_user = find_user_by_username(db, username).await;
        if let Some(user) = maybe_user {
            Ok(user.to_user_info())
        } else {
            Err("No user found".into())
        }
    }
    */
}
