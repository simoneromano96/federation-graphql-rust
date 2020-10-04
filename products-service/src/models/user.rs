use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};

/// User representation
#[derive(Debug, Serialize, Deserialize)]
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
