use serde::{Deserialize, Serialize};
use paperclip::actix::Apiv2Schema;

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub enum Role {
    Guest,
    Customer,
    Admin,
}
