use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub enum Role {
    Guest,
    Customer,
    Admin,
}
