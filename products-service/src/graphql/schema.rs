use super::{Mutation, Query};
use async_graphql::{EmptySubscription, Schema};

pub type ProductsServiceSchema = Schema<Query, Mutation, EmptySubscription>;
