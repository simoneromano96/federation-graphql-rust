use paperclip::actix::{
    Apiv2Schema,
};
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

/// User representation
#[derive(Debug, Model, Serialize, Deserialize, Apiv2Schema)]
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

impl User {
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
        }
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
