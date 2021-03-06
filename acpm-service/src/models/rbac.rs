use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct AddRolesForUser {
    /// User's ID
    pub user_id: String,
    /// Roles
    pub roles: Vec<String>,
    /// Domain of the role, ex. app-1, app-2
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    /// The action that the role is permitted to do, ex. read, create, update, delete
    pub action: String,
    /// The object that receives the action, ex. product, post, coffee, etc.
    pub object: String,    
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct AddPermissionToRole {
    /// Role name, ex. admin, customer, guest
    pub role: String,
    /// Permissions granted to the role
    pub permissions: Vec<Permission>
}
