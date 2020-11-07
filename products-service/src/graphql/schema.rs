use super::{Mutation, Query, Subscription};
use async_graphql::{Schema};

pub type ProductsServiceSchema = Schema<Query, Mutation, Subscription>;
