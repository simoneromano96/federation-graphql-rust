use crate::config::APP_CONFIG;
use async_graphql::{async_trait, guard::Guard};
use async_graphql::{Context, Result};
use async_trait::async_trait;
use log::info;
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
        if APP_CONFIG.authorization.skip {
            info!("Skipping authorization check");
            Ok(())
        } else {
            let client: &reqwest::Client = ctx.data().unwrap();
            let mut user = "guest";
    
            if let Some(logged_user) = ctx.data_opt::<String>() {
                user = logged_user;
            }
    
            let subject = user;
            let action;
            let object;

            info!("{:?}", serde_json::to_string(&self.permission));
    
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
    
            info!("Requesting access to resource");
            info!("{:?}::{:?}::{:?}", subject, action, object);
    
            // The permission query is formed as: 
            // subject (user role, default to guest) 
            // action (create, read, update, delete) 
            // object (coffee)
            let request = client
                .get(&APP_CONFIG.authorization.url)
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
}
