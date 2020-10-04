use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use super::Query;

pub type IdentityServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
