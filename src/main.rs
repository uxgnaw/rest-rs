#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

mod db;
mod handler;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=error,info");
    // env_logger::init();
    dotenv::dotenv().ok();

    WriteLogger::init(LevelFilter::Info, Config::default(),
                                       File::create("info.log").unwrap()).unwrap();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(handler::user_get)
            .service(handler::user_add)
            .route(
                "/",
                web::get().to(|| HttpResponse::Ok().body("hello, world!")),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound().body("404")))
    })
    // .workers(10)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
