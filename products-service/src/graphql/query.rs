use crate::models::{Coffee, CreateCoffeeInput, UpdateCoffeeInput};
use crate::{
    authorization::{Permission, PermissionGuard},
    models::coffee::CoffeeChanged,
    models::coffee::MutationType,
};
use async_graphql::guard::Guard;
use async_graphql::{Context, Object, Result, ID, Subscription};
use futures::{Stream, stream::StreamExt};
use log::info;
// use redis_async::{resp::FromResp, client::{PairedConnection, PubsubConnection}, resp_array};
use redis::AsyncCommands;
use serde_json;
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
};

async fn fetch_all_coffees(db: &Database) -> Result<Vec<Coffee>> {
    info!("Fetching all coffees");
    let mut coffees: Vec<Coffee> = Vec::new();

    let mut cursor = Coffee::find(db, None, None).await?;

    while let Some(coffee) = cursor.next().await {
        coffees.push(coffee.unwrap());
    }

    Ok(coffees)
}

async fn fetch_coffee_by_id(db: &Database, id: String) -> Result<Coffee> {
    info!("Fetching a coffee with id: {:?}", id);

    let query = doc! {
        "_id": ObjectId::with_string(&id)?,
    };

    if let Some(coffee_model) = Coffee::find_one(db, Some(query), None).await? {
        Ok(coffee_model)
    } else {
        Err(format!("Coffee with ID {:?} not found", id).into())
    }
}

async fn create_coffee(
    db: &Database,
    redis_client: &redis::Client,
    input: CreateCoffeeInput,
) -> Result<Coffee> {
    let mut coffee = Coffee {
        id: None,
        name: input.name,
        price: input.price,
        image_url: input.image_url.into_string(),
        description: input.description,
    };

    coffee.save(db, None).await?;

    let message = CoffeeChanged {
        mutation_type: MutationType::Created,
        id: ID::from(coffee.id.clone().unwrap().to_string()),
    };

    let json = serde_json::to_string(&message)?;

    let mut publish_conn = redis_client.get_async_connection().await?;
    publish_conn.publish("coffees", &json).await?;
    // redis_connection.send_and_forget(resp_array!["PUBLISH", "coffees", &json]);

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

    if let Some(mut coffee) = Coffee::find_one(db, Some(query), None).await? {
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

pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    /// Returns an array with all the coffees or an empty array
    #[graphql(guard(PermissionGuard(permission = "Permission::ReadCoffee")))]
    async fn coffees(&self, ctx: &Context<'_>) -> Result<Vec<Coffee>> {
        let db: &Database = ctx.data()?;
        fetch_all_coffees(db).await
    }

    /// Returns a coffee by its ID, will return error if none is present with the given ID
    #[graphql(guard(PermissionGuard(permission = "Permission::ReadCoffee")))]
    async fn coffee(&self, ctx: &Context<'_>, id: ID) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        fetch_coffee_by_id(db, id.to_string()).await
    }

    /// Returns a coffee by its ID, will return error if none is present with the given ID
    #[graphql(entity)]
    async fn find_coffee_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        fetch_coffee_by_id(db, id.to_string()).await
    }
}

pub struct Mutation;

#[Object(extends, cache_control(max_age = 60))]
impl Mutation {
    /// Creates a new coffee
    #[graphql(guard(PermissionGuard(permission = "Permission::CreateCoffee")))]
    async fn create_coffee(&self, ctx: &Context<'_>, input: CreateCoffeeInput) -> Result<Coffee> {
        // let redis_pubsub_connection: &PubsubConnection = ctx.data()?;
        // let (redis_connection, _): &(PairedConnection, PubsubConnection) = ctx.data()?;
        let redis_client: &redis::Client = ctx.data()?;

        let db: &Database = ctx.data()?;

        create_coffee(db, redis_client, input).await
    }

    /// Updates a coffee
    #[graphql(guard(PermissionGuard(permission = "Permission::UpdateCoffee")))]
    async fn update_coffee(&self, ctx: &Context<'_>, input: UpdateCoffeeInput) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        update_coffee(db, input).await
    }

    /// Deletes a coffeee
    #[graphql(guard(PermissionGuard(permission = "Permission::DeleteCoffee")))]
    async fn delete_coffee(&self, ctx: &Context<'_>, id: String) -> Result<Coffee> {
        let db: &Database = ctx.data()?;
        delete_coffee(db, id).await
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn coffees(
        &self,
        ctx: &Context<'_>,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CoffeeChanged> {
        // let (_, pubsub_connection): &(PairedConnection, PubsubConnection) = ctx.data().unwrap();
        let redis_client: &redis::Client = ctx.data().unwrap();

        let mut pubsub_conn = redis_client.get_async_connection().await.unwrap().into_pubsub();

        pubsub_conn.subscribe("coffees").await.expect("Cannot subscribe to topic");
        let mut pubsub_stream = pubsub_conn.into_on_message();

        pubsub_stream.filter_map(move |msg| {
            let mut res = None;
            if let Some(mutation_type) = mutation_type {
                if let Ok(payload) = msg.get_payload::<String>() {
                    let coffee_changed: CoffeeChanged = serde_json::from_str(&payload).unwrap();
                    if coffee_changed.mutation_type == mutation_type {
                        res = Some(coffee_changed)
                    }
                }
            }
            async { res }
        })

        /*
        let msgs = pubsub_connection
            .subscribe("coffees")
            .await
            .expect("Cannot subscribe to topic");

        msgs.filter_map(move |e| {
            let mut res = None;
            if let Ok(resp) = e {
                if let Some(mutation_type) = mutation_type {
                    let msg: CoffeeChanged =
                        serde_json::from_str(&(String::from_resp(resp).unwrap())).unwrap();
                    if msg.mutation_type == mutation_type {
                        res = Some(msg)
                    }
                }
            }
            async move { res }
        })
        */
        // SimpleBroker::<CoffeeChanged>::subscribe().filter(move |event| {
        //     let res = if let Some(mutation_type) = mutation_type {
        //         event.mutation_type == mutation_type
        //     } else {
        //         true
        //     };
        //     async move { res }
        // })
    }
}
