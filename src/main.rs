mod data;
mod db;
mod service;
// mod errors;

use actix_web::{App,HttpResponse, HttpServer, Responder, get, middleware::Logger};
use futures::{stream::TryStreamExt};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[actix_web::main]
async fn main()  -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "rsvp=debug");

    let client = db::connect().await;

    println!("Listening on port 8080");
    HttpServer::new(move || 
        App::new() 
        .wrap(Logger::default())
        .data(client.clone())
        .service(hello)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
