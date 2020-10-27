use super::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type IdentityServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;
