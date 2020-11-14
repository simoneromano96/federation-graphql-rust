use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct AddUserToRole {
    pub user_id: String,
    pub role: String,
    pub domain: Option<String>,
}
