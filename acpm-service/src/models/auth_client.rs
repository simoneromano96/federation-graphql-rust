use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct AuthClient { 
    pub id: i32, 
    pub client_id: String, 
    pub client_secret: String,
}
