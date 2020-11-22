use super::ProductsServiceSchema;
use actix_session::Session;
use actix_web::{
    web::{Data, Payload},
    HttpRequest, HttpResponse, Result,
};
use actix_web_actors::ws;
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};
// use wither::bson::oid::ObjectId;

pub async fn index(
    schema: Data<ProductsServiceSchema>,
    // req: HttpRequest,
    gql_request: Request,
    session: Session,
    // http_client: Data<Client>,
) -> Response {
    let maybe_user: Option<String> = session.get("user_id").unwrap_or(None);
    // let maybe_user_role: Option<String> = session.get("user_role").unwrap_or(None);

    // println!("{:?}", maybe_user);

    let mut request = gql_request.into_inner();
    if let Some(user_id) = maybe_user {
        // println!("Add User Info: id: {:?}", &user_id);
        request = request.data(user_id);
    }
    // if let Some(user_role) = maybe_user_role {
    //     // println!("Add User Info: role: {:?}", &user_role);
    //     request = request.data(user_role);
    // }

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

/*
pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        ))
}
*/
