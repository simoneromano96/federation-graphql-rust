use async_graphql::{Schema, EmptyMutation};
use super::{Subscription, Query};

pub type SubscriptionServiceSchema = Schema<Query, EmptyMutation, Subscription>;
