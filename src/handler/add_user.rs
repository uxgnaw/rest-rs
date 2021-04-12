use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::db;
use crate::models;

#[get("/add/user/{name}")]
pub async fn add_user(name: web::Path<String>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let nm = name.clone();
    let user = web::block(|| insert_new_user(nm)).await.map_err(|e| {
        eprintln!("{:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    info!("add user {} success", name);
    Ok(HttpResponse::Ok().json(user))
}

fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: String,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let conn = db::get_conn().expect("couldn't get db connection from pool");
    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)?;

    Ok(new_user)
}
