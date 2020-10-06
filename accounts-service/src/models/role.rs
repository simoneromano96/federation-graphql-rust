use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Guest,
    Customer,
    Admin,
}
