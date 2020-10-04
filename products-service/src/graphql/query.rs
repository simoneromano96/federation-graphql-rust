use async_graphql::{Context, Object, Result, ID};
use async_graphql::guard::Guard;
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
// use bson::doc;
use crate::authorization::{PermissionGuard, Permission};
use crate::models::{Coffee, CreateCoffeeInput, UpdateCoffeeInput};
// use futures::{Stream, StreamExt};
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
};
// use std::iter::Iterator;
use futures::stream::StreamExt;

pub struct Query;

async fn fetch_all_coffees(db: &Database) -> Result<Vec<Coffee>> {
    let mut coffees: Vec<Coffee> = Vec::new();

    let mut cursor = Coffee::find(db, None, None).await?;

    while let Some(coffee) = cursor.next().await {
        coffees.push(coffee.unwrap());
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(db: &Database, id: String) -> Result<Coffee> {
    let query = doc! {
        "_id": ObjectId::with_string(&id)?,
    };

    if let Some(coffee_model) = Coffee::find_one(db, Some(query), None).await? {
        Ok(coffee_model)
    } else {
        Err(format!("Coffee with ID {:?} not found", id).into())
    }
}

async fn create_coffee(db: &Database, input: CreateCoffeeInput) -> Result<Coffee> {
    let mut coffee = Coffee {
        id: None,
        name: input.name,
        price: input.price,
        image_url: input.image_url.into_string(),
        description: input.description,
    };

    coffee.save(db, None).await?;

    // SimpleBroker::publish(CoffeeChanged {
    //     mutation_type: MutationType::Created,
    //     id: ID::from(coffee.id.clone().unwrap().to_string()),
    // });

    Ok(coffee)
}

async fn update_coffee(db: &Database, input: UpdateCoffeeInput) -> Result<Coffee> {
    let id = input.id;

    let query = doc! {
        "_id": ObjectId::with_string(&id)?
    };

    if let Some(mut coffee) = Coffee::find_one(db, Some(query), None)
    .await? {
        if let Some(name) = input.name {
            coffee.name = name;
        }
    
        if let Some(price) = input.price {
            coffee.price = price;
        }
    
        if let Some(description) = input.description {
            coffee.description = Some(description);
        }
    
        if let Some(image_url) = input.image_url {
            coffee.image_url = image_url.to_string();
        }
    
        coffee.save(db, None).await?;
    
        Ok(coffee)
    } else {
        Err(format!("Coffee with id: {:?} not found", id).into())
    }
}

async fn delete_coffee(db: &Database, id: String) -> Result<Coffee> {
    let query = doc! {
        "_id": ObjectId::with_string(&id)?
    };

    let res: Option<Coffee> = Coffee::find_one_and_delete(db, query, None).await?;

    if let Some(coffee) = res {
        // SimpleBroker::publish(CoffeeChanged {
        //     mutation_type: MutationType::Deleted,
        //     id: ID::from(coffee.id.clone().unwrap().to_string()),
        // });

        Ok(coffee)
    } else {
        Err(format!("Coffee with ID {:?} not found", id).into())
    }
}

#[Object(extends)]
impl Query {
    /// Returns an array with all the coffees or an empty array
    async fn coffees(&self, ctx: &Context<'_>) -> Result<Vec<Coffee>> {
        let db: &Database = ctx.data()?;

        fetch_all_coffees(db).await
    }

    /// Returns a coffee by its ID, will return error if none is present with the given ID
    async fn coffee(&self, ctx: &Context<'_>, id: String) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        fetch_coffee_by_id(db, id).await
    }

    #[graphql(entity)]
    async fn echo(&self, name: String, price: f64, image_url: String) -> Coffee {
        Coffee {
            id: None,
            name,
            description: None,
            price,
            image_url,
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// Creates a new coffee
    // #[graphql(entity = true, external = false, provides = "createCoffee")]
    #[graphql(guard(PermissionGuard(permission = "Permission::CreateCoffee")))]
    async fn create_coffee(&self, ctx: &Context<'_>, input: CreateCoffeeInput) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        create_coffee(db, input).await
    }

    /// Updates a coffee
    // #[graphql(entity = true, external = false, provides = "updateCoffee")]
    async fn update_coffee(&self, ctx: &Context<'_>, input: UpdateCoffeeInput) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        update_coffee(db, input).await
    }

    /// Deletes a coffeee
    // #[graphql(entity = true, external = false, provides = "deleteCoffee")]
    async fn delete_coffee(&self, ctx: &Context<'_>, id: String) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        delete_coffee(db, id).await
    }

    #[graphql(entity)]
    async fn echo_create_input(&self, _input: CreateCoffeeInput) -> Result<String> {
        Ok("ok".into())
    }

    #[graphql(entity)]
    async fn echo_update_input(&self, _input: UpdateCoffeeInput) -> Result<String> {
        Ok("ok".into())
    }

    #[graphql(entity)]
    async fn echo_delete(&self, _id: String) -> Result<String> {
        Ok("ok".into())
    }
}

/*
#[async_graphql::Enum]
#[derive(Debug)]
enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
struct CoffeeChanged {
    mutation_type: MutationType,
    id: ID,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn coffees(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CoffeeChanged> {
        SimpleBroker::<CoffeeChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
*/
