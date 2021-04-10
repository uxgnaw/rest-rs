#[macro_use]
extern crate diesel;

use actix_web::{App, web, HttpServer, middleware, HttpResponse};

mod models;
mod handler;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=error,info");
    env_logger::init();
    dotenv::dotenv().ok();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(handler::user_get)
            .service(handler::user_add)
            .route("/", web::get().to(|| HttpResponse::Ok().body("hello, world")))
            .default_service(web::route().to(|| HttpResponse::NotFound().body("404")))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}