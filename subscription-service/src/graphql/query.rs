use async_graphql::{Context, Object, Result, Subscription};
use redis_async::{client::PubsubConnection, resp::FromResp};
// use redis_async::{client::PubsubConnection, resp::FromResp};
// use nanoid::nanoid;
// use serde::ser::SerializeStruct;
// use bson::doc;
use crate::{models::CoffeeChanged, models::MutationType};
// use futures::{Stream, StreamExt};
use serde_json;
// use std::iter::Iterator;
use futures::{stream::StreamExt, Stream};

pub struct Query;

#[Object(extends)]
impl Query {
    async fn ping(&self) -> Result<String> {
        Ok(String::from("pong"))
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
        let pubsub_connection: &PubsubConnection = ctx.data().unwrap();

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
