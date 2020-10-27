use super::IdentityServiceSchema;
use crate::models::User;
use actix_session::Session;
use actix_web::{web::Data, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use wither::{bson::oid::ObjectId, mongodb::Database};

pub async fn index(
    schema: Data<IdentityServiceSchema>,
    // req: HttpRequest,
    db: Data<Database>,
    gql_request: Request,
    session: Session,
) -> Response {
    let mut request = gql_request.into_inner();
    if let Some(id) = session.get::<ObjectId>("user_id").unwrap_or(None) {
        let user = User::find_by_id(&db, &id).await.unwrap();
        request = request.data(user);
    }

    schema.execute(request).await.into()
}

pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        ))
}
