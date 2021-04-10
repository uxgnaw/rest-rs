use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: r2d2::Pool<ConnectionManager<diesel::MysqlConnection>> = {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
        r2d2::Pool::builder()
            .max_size(200)
            .min_idle(Some(10))
            .build(manager)
            .expect("Failed to create pool.")
    };
}
