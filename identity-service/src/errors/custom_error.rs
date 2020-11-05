
#[api_v2_errors(
    code = 400,
    description = "Bad Request: Errors in the body",
    code = 401,
    description = "Unauthorized: Can't read session from cookie",
    code = 500
)]
#[derive(Debug, Serialize, Deserialize)]
pub enum CustomError {
    BadRequest(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        write!(&mut s, "{:?}", self)
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CustomError::BadRequest(_) => {
                actix_web::http::StatusCode::BAD_REQUEST
            }
        }
        // actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = HttpResponse::new(self.status_code());
        resp.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        let mut buf = web::BytesMut::new();

        match self {
            CustomError::BadRequest(message) => {
                write!(&mut buf, "{:?}", message).unwrap();
            }
        }
        // let _ = write!(Writer(&mut buf), "{}", self);
        resp.set_body(actix_web::dev::Body::from(buf))
    }
}
