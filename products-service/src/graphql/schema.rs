use async_graphql::{EmptySubscription, Schema};
use super::{Query, Mutation, Subscription};

pub type ProductsServiceSchema = Schema<Query, Mutation, Subscription>;
