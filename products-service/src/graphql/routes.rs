use actix_session::Session;
use actix_web::{client::Client, HttpRequest, HttpResponse, Result, web::{Data, Payload}};
use actix_web_actors::ws;
// use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{Schema, http::{GraphQLPlaygroundConfig, playground_source}};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use wither::bson::oid::ObjectId;

use super::ProductsServiceSchema;
// use crate::models::User;

pub async fn index(
    schema: Data<ProductsServiceSchema>,
    // req: HttpRequest,
    gql_request: Request,
    session: Session,
    // http_client: Data<Client>,
) -> Response {
    let maybe_user: Option<ObjectId> = session.get("user_id").unwrap_or(None);

    // println!("{:?}", maybe_user);

    let mut request = gql_request.into_inner();
    if let Some(user) = maybe_user {
        request = request.data(user);
    }

    schema.execute(request).await.into()
}

pub async fn index_ws(
    schema: Data<ProductsServiceSchema>,
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

pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        ))
}
