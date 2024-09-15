use actix_web::{get, web, App, HttpServer, Responder};
use tracing::Level;
use tracing_subscriber;

#[get("/")]
async fn hello() -> impl Responder {
    web::Json("Hello from Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
