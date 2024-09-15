use actix_web::{App, HttpServer};
use tracing::{info, Level};
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // !!!
    // Comment out the following lines because we dont have services yet
    //
    // HttpServer::new(|| {
    //     App::new()
    //         .service(/* your services here */)
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
    Ok(())
}
