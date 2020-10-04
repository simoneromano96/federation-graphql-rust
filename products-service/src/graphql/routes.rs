use actix_session::Session;
use actix_web::web::Data;
// use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{Request, Response};

use super::ProductsServiceSchema;
use crate::models::User;

pub async fn index(
    schema: Data<ProductsServiceSchema>,
    // req: HttpRequest,
    gql_request: Request,
    session: Session,
) -> Response {
    let maybe_user: Option<User> = session.get("user").unwrap_or(None);

    // println!("{:?}", maybe_user);

    let mut request = gql_request.into_inner();
    if let Some(user) = maybe_user {
        request = request.data(user);
    }

    schema.execute(request).await.into()
}

/*
pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}
*/
