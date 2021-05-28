use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use ::r2d2::Error as r2d2Error;
use once_cell::sync::OnceCell;

static POOL: OnceCell<r2d2::Pool<ConnectionManager<diesel::MysqlConnection>>> = OnceCell::new();

pub fn get_conn(
) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, r2d2Error> {
    let pool = POOL.get_or_init(|| {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
        r2d2::Pool::builder()
            .max_size(200)
            .min_idle(Some(10))
            .build(manager)
            .expect("Failed to create pool.")
    });

    pool.clone().get()
}
