use actix_web::{Error, get, HttpResponse, web};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db;
use crate::models;

#[get("/add/user/{name}")]
pub async fn add_user(
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || insert_new_user(&name))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let conn = db::POOL.clone().get().expect("couldn't get db connection from pool");
    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(&conn)?;

    Ok(new_user)
}

