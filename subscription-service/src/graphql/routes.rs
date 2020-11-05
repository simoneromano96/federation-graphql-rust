use actix_web::{
    web::{Data, Payload},
    HttpRequest, HttpResponse, Result,
};
use actix_web_actors::ws;
// use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{
    Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use super::SubscriptionServiceSchema;
// use crate::models::User;

pub async fn index(
    schema: Data<SubscriptionServiceSchema>,
    // req: HttpRequest,
    gql_request: Request,
    // http_client: Data<Client>,
) -> Response {
    // println!("{:?}", maybe_user);
    let request = gql_request.into_inner();
    schema.execute(request).await.into()
}

pub async fn index_ws(
    schema: Data<SubscriptionServiceSchema>,
    req: HttpRequest,
    payload: Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}
