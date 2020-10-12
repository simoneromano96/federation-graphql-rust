use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    // mongodb::Database,
};

use super::{Action, Object, Role};

/// Permission representation
#[derive(Debug, Model, Serialize, Deserialize)]
#[model()]
pub struct Permission {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Some role
    pub role: Role,
    pub actions: Vec<Action>,
    pub objects: Vec<Object>,
}
