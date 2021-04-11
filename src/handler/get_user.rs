use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db;
use crate::models;

#[get("/user/{user_id}")]
pub async fn get_user(user_uid: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || find_user_by_uid(user_uid))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

fn find_user_by_uid<'a>(uid: Uuid) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = db::POOL.clone().get().expect("can't ge db conn from pool");

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(&conn)
        .optional()?;

    Ok(user)
}
