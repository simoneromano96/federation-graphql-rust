use crate::models::AuthClient;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config},
    AuthenticationError,
};
use log::info;
use sqlx::{Pool, Postgres};

const CLIENT_QUERY: &str = r#"
    SELECT *
    FROM "clients"
        WHERE 
            "client_id" = $1 AND
            "client_secret" = $2
    LIMIT 1
"#;

async fn validate_credentials(pool: &Pool<Postgres>, client_id: &str, client_secret: &str) -> bool {
    info!("Authenticating: {:?} - {:?}", client_id, client_secret);

    let client: Option<AuthClient> = sqlx::query_as::<_, AuthClient>(CLIENT_QUERY)
        .bind(client_id)
        .bind(client_secret)
        .fetch_optional(pool)
        .await
        .unwrap();

    info!("{:?}", client);

    if let Some(_) = client {
        true
    } else {
        false
    }
}

pub async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    info!("Requested basic auth");
    
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let pool = req.app_data::<Pool<Postgres>>().expect("No pool found");

    if validate_credentials(
        pool,
        credentials.user_id(),
        credentials.password().unwrap().trim(),
    )
    .await
    {
        Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}
