use async_graphql::{Context, Object, ID};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
};

use super::Role;

/// User representation
#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The username.
    pub username: String,
    /// The hashed password.
    pub password: String,
    // User email
    // email: String,
    /// User role
    pub role: String,
}

impl User {
    pub fn new_user(username: &str, password: &str) -> Self {
        User {
            id: None,
            username: String::from(username),
            password: String::from(password),
            role: String::from("customer"),
        }
    }

    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
        }
    }

    pub async fn find_by_id(db: &Database, id: &ObjectId) -> Option<Self> {
        User::find_one(&db, doc! { "_id": id }, None).await.unwrap()
    }

    pub async fn find_by_username(db: &Database, username: &str) -> Option<Self> {
        User::find_one(&db, doc! { "username": username }, None)
            .await
            .unwrap()
    }
}

/// Available User info
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct UserInfo {
    /// The ID of the user.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The username.
    pub username: String,
}

#[Object]
impl UserInfo {
    async fn id(&self) -> String {
        if let Some(id) = &self.id {
            id.clone().to_string()
        } else {
            String::from("")
        }
    }

    async fn username(&self) -> &str {
        &self.username
    }
}

/// New User Input
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct UserInput {
    /// The new user username, must be unique.
    pub username: String,
    /// The new user password.
    pub password: String,
    // User email
    // email: String,
}
