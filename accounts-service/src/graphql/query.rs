use async_graphql::{Context, ID, Object, Result};

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

    #[graphql(entity)]
    async fn echo(&self, username: String) -> UserInfo { UserInfo { id: None, username } }
}
