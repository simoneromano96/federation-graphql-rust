use serde::{Deserialize, Serialize};
use wither::bson::oid::ObjectId;

use super::{Action, Object};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionQuery {
    pub subject: ObjectId,
    pub action: String,
    pub object: String,
}
