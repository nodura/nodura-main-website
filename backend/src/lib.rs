use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    web::Json("Hello from Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
