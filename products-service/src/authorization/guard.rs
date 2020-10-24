// use crate::models::User;
use async_graphql::{async_trait, guard::Guard};
use async_graphql::{Context, Result};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use wither::bson::oid::ObjectId;

use crate::config::APP_CONFIG;

use crate::utils::http_client::get_http_client;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    CreateCoffee,
    UpdateCoffee,
    DeleteCoffee,
    // Other(String),
}

pub struct PermissionGuard {
    pub permission: Permission,
}

#[async_trait]
impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let client: &reqwest::Client = ctx.data().unwrap();
        
        if let Some(user_id) = ctx.data_opt::<ObjectId>() {
            // let client = get_http_client();
            // let r = client.get(url).send().await;
            let subject = &user_id.to_string();
            let action;
            let object;

            // TODO: I don't really like this
            match self.permission {
                Permission::CreateCoffee => {
                    object = "coffee";
                    action = "create";
                }
                Permission::UpdateCoffee => {
                    object = "coffee";
                    action = "update";
                }
                Permission::DeleteCoffee => {
                    object = "coffee";
                    action = "delete";
                }
            }

            let request = client
                .get(&APP_CONFIG.authorization_server.url)
                .query(&[
                    ("subject", String::from(subject)),
                    ("action", String::from(action)),
                    ("object", String::from(object)),
                ])
                .build()?;

            let res = client.execute(request).await?;

            // println!("{:?}", res);

            let status = res.status();
            match status {
                StatusCode::OK => Ok(()),
                _ => Err("Cannot access resource".into()),
            }
        } else {
            Err("Must be authenticated".into())
        }
    }
}
