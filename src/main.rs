use actix_web::{App,HttpResponse, HttpServer, Responder, get};
mod data;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

#[actix_web::main]
async fn main()  -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "rsvp=debug");

    HttpServer::new(move || 
        App::new() 
        .service(hello)   
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
