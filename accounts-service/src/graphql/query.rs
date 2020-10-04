use async_graphql::{Context, ID, Object, Result};

use crate::models::{User, UserInfo};

pub struct Query;

#[Object(extends)]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<UserInfo> {
        // let user = ctx.data::<User>();
        if let Ok(user) = ctx.data::<User>() {
            Ok(user.to_user_info())
        } else {
            Err("Not logged in".into())
        }
    }
    
    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> UserInfo {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        UserInfo { id: None, username }
    }
}