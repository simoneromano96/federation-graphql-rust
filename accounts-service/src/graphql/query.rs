use async_graphql::{Context, Object, Result, ID};
use wither::Model;
use wither::{bson::doc, bson::oid::ObjectId, mongodb::Database};

use crate::models::{User, UserInfo};

pub struct Query;

#[Object(extends)]
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

    /// Get a user by its ID
    #[graphql(entity)]
    async fn find_user_info_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<UserInfo> {
        let oid_result = ObjectId::with_string(&id.to_string());
        if let Ok(oid) = oid_result {
            let db: &Database = ctx.data()?;
            let maybe_user = User::find_by_id(db, &oid).await;
            if let Some(user) = maybe_user {
                Ok(user.to_user_info())
            } else {
                Err("No user found".into())
            }
        } else {
            Err("Invalid ID".into())
        }
    }
}
