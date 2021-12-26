// use actix_web::{error::ResponseError, HttpResponse};
use dotenv;
use mongodb::{Client};

// pub async fn connect() -> mongodb::error::Result<Client> {
pub async fn connect() -> Client {

    let url = dotenv::var("DATABASE_URL")
        .unwrap_or_else(|_| "mongodb:://localhost:27017".into());

    let client = Client::with_uri_str(url).await.expect("Error connecting to MongoDB client");

    return client;
}