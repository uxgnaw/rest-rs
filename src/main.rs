#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};

mod models;
mod handler;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=error,info");
    env_logger::init();
    dotenv::dotenv().ok();

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(handler::user_get)
            .service(handler::user_add)
    })
        .bind(&bind)?
        .run()
        .await
}