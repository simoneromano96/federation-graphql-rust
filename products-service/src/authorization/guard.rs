use crate::config::APP_CONFIG;
use async_graphql::{async_trait, guard::Guard};
use async_graphql::{Context, Result};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    CreateCoffee,
    ReadCoffee,
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
        let mut user_role = "guest";

        if let Some(logged_user_role) = ctx.data_opt::<String>() {
            user_role = logged_user_role;
        }

        let subject = user_role;
        let action;
        let object;

        // TODO: I don't really like this
        match self.permission {
            Permission::CreateCoffee => {
                object = "coffee";
                action = "create";
            }
            Permission::ReadCoffee => {
                object = "coffee";
                action = "read";
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

        // println!("Requesting access to resource");
        // println!("{:?}::{:?}::{:?}", subject, action, object);

        let request = client
            .get(&APP_CONFIG.authorization_server.url)
            .query(&[
                ("subject", String::from(subject)),
                ("action", String::from(action)),
                ("object", String::from(object)),
            ])
            .build()?;

        let res = client.execute(request).await?;

        // println!("Authorized response: {:?}", res);

        let status = res.status();
        match status {
            StatusCode::OK => Ok(()),
            _ => Err("Cannot access resource".into()),
        }
    }
}
