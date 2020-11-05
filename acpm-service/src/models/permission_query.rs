use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct PermissionQuery {
    pub subject: String,
    pub action: String,
    pub object: String,
}
