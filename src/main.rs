mod data;
mod db;
mod service;
mod errors;


use actix_cors::Cors;
use actix_web::{web, web::Json,App,HttpResponse, HttpServer, Responder, get, middleware::Logger, HttpRequest};
// use futures::{stream::TryStreamExt};
use mongodb::bson::DateTime;


use data::models::user::{User,Contact,Membership};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

async fn dummy_api(req : HttpRequest) -> Result<Json<User>, errors::UserError> {
    let dummy_contact = Contact {
        email: "foo@bar.com".to_string(),
        // phone: Some("123-456-7890".to_string()),
        phone: None
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
    Ok(Json(dummy_user))

}

#[actix_web::main]
async fn main()  -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "rsvp=debug");

    let client = db::connect().await;

    println!("Listening on port 8080");
    // println!("Listening on port 3000");
    HttpServer::new(move || 
        App::new() 
        .wrap(Logger::default())
        .wrap(Cors::permissive())
        .data(client.clone())
        .service(hello)
        .service(
            web::scope("/api").route(
                "/dummy_user",
                web::get().to(dummy_api)
            )
        )
    )
    .bind("127.0.0.1:8080")?
    // .bind("127.0.0.1:3000")?
    .run()
    .await
}
