use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{AuthenticationError, basic::{BasicAuth, Config}};

async fn validate_credentials(client_id: String, client_password: String) -> Result<bool, bool> {
    
}

async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match validate_credentials(
        credentials.user_id().to_string(),
        credentials.password().unwrap().trim().to_string(),
    ).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
