mod data;
mod db;
mod service;
// mod errors;

#[macro_use] extern crate rocket;
extern crate r2d2;

// use actix_cors::Cors;
// use actix_web::{web, web::Json,App,HttpResponse, HttpServer, Responder, get, middleware::Logger, HttpRequest};
// use futures::{stream::TryStreamExt};
use rocket::serde::json::Json;
use rocket_cors::CorsOptions;
use std::error::Error;

use mongodb::{bson::DateTime, Database};
use data::models::user::{User,Contact,Membership};

struct AppDataPool {
    mongo: Database
}


#[get("/")]
// async fn hello() -> impl Responder {
async fn hello() -> Json<String> {
    Json("Hello world!".to_owned())
}
#[get("/dummy_user")]
// async fn dummy_api(req : HttpRequest) -> Result<Json<User>, errors::UserError> {
async fn dummy_api() -> Json<User>{
    let dummy_contact = Contact {
        email: "foo@bar.com".to_string(),
        phone: Some("123-456-7890".to_string()),
    };
    
    let dummy_membership = Membership {
        id: 54321,
        location: vec!["Stanford".to_owned(), "Berkeley".to_owned()],
        start: DateTime::now(),
        exp: None,
    };


    let dummy_user = User::new(
        dummy_contact,
        "password".to_owned(),
        "foo".to_owned(),
        "bar".to_owned(),
        Some("foobar123".to_owned()),
        Some(dummy_membership),

    );
    // Some(Json(dummy_user))
    Json(dummy_user)
}

// #[actix_web::main]
#[rocket::main]
// async fn main()  -> std::io::Result<()>{
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "rsvp=debug");

    let client = db::connect().await;

    println!("Listening on port 8000");
    // println!("Listening on port 8080");
    // println!("Listening on port 3000");
    let _ = rocket::build()
        // .manage(database)
        .mount("/", routes![hello])
        .mount("/api", routes![dummy_api])
        .attach(CorsOptions::default().to_cors()?)
        .launch()
        .await;
    // HttpServer::new(move || 
    //     App::new() 
    //     .wrap(Logger::default())
    //     .wrap(Cors::permissive())
    //     .data(client.clone())
    //     .service(hello)
    //     .service(
    //         web::scope("/api").route(
    //             "/dummy_user",
    //             web::get().to(dummy_api)
    //         )
    //     )
    // )
    // .bind("127.0.0.1:8080")?
    // .bind("127.0.0.1:3000")?
    // .run()
    // .await
    Ok(())
}
