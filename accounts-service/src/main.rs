use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{cookie, middleware, web, App, HttpResponse, HttpServer};
use actix_web::{guard, HttpRequest};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, Object, Schema, SimpleObject};
use async_graphql::{EmptySubscription, ID};
use async_graphql_actix_web::{Request, Response};

#[derive(SimpleObject)]
struct User {
    id: ID,
    username: String,
}

struct Query;

#[Object(extends)]
impl Query {
    async fn me(&self) -> User {
        User {
            id: "1234".into(),
            username: "Me".to_string(),
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        User { id, username }
    }
}

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: HttpRequest,
    gql_request: Request,
    session: Session,
) -> Response {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    println!("User ID: {:?}", user_id);

    schema.execute(gql_request.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema: Schema<Query, EmptyMutation, EmptySubscription> =
        Schema::new(Query, EmptyMutation, EmptySubscription);

    println!("Playground: http://127.0.0.1:4001");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // cookie session middleware
            .wrap(
                RedisSession::new("127.0.0.1:6379", b"N7WoK3mG7lSb0CpK8UhAabUZNi27n5ub")
                    // Don't allow the cookie to be accessed from javascript
                    .cookie_http_only(true)
                    // allow the cookie only from the current domain
                    .cookie_same_site(cookie::SameSite::Lax),
            )
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            // .service(
            //     web::resource("/")
            //         .guard(guard::Get())
            //         .guard(guard::Header("upgrade", "websocket"))
            //         .to(index_ws),
            // )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("0.0.0.0:4001")?
    .run()
    .await
}
