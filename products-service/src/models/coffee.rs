use async_graphql::{self, Enum, InputObject, Object, ID};
use serde::{Deserialize, Serialize};
use url::Url;
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

/// Define the Coffee Model
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(
    collection_name = "coffees",
    index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#)
)]
#[serde(rename_all = "camelCase")]
pub struct Coffee {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Coffee's name.
    pub name: String,
    /// Coffee's price.
    pub price: f64,
    /// Coffee's image.
    pub image_url: String,
    /// Coffee's description (optional).
    pub description: Option<String>,
}

#[Object]
impl Coffee {
    async fn id(&self) -> String {
        if let Some(id) = &self.id {
            id.clone().to_string()
        } else {
            String::from("")
        }
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn price(&self) -> &f64 {
        &self.price
    }

    async fn image_url(&self) -> &str {
        &self.image_url
    }

    async fn description(&self) -> String {
        if let Some(description) = &self.description {
            description.clone()
        } else {
            String::from("")
        }
    }
}

#[derive(Clone, InputObject)]
pub struct CreateCoffeeInput {
    pub name: String,
    pub price: f64,
    pub image_url: Url,
    pub description: Option<String>,
}

#[derive(Clone, InputObject)]
pub struct UpdateCoffeeInput {
    pub id: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<Url>,
    pub description: Option<String>,
}

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
