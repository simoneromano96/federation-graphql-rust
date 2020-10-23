use serde::{Deserialize, Serialize};
use async_graphql::{self, Enum, InputObject, Object, ID};

#[derive(Debug, Enum, Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoffeeChanged {
    pub id: ID,
    pub mutation_type: MutationType,
}

#[Object]
impl CoffeeChanged {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }
}
