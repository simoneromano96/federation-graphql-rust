use async_graphql::{EmptySubscription, Schema};
use super::{Query, Mutation};

pub type ProductsServiceSchema = Schema<Query, Mutation, EmptySubscription>;
