use reqwest::{Client, ClientBuilder, header};

/*
use actix_web::client::{Client, ClientBuilder};

pub fn get_http_client() -> Client {
    ClientBuilder::new()
        .basic_auth("service", Some("password"))
        .finish()
}
*/

pub fn get_http_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("secret"));

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Could not create http client")
}