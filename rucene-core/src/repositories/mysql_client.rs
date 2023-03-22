use std::time::Duration;
use diesel::{MysqlConnection, r2d2};
use diesel::r2d2::{ConnectionManager, Pool};

pub struct MysqlClient {
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

impl MysqlClient {
    pub fn new() -> MysqlClient {
        let manager = ConnectionManager::<MysqlConnection>::new("");
        let pool = r2d2::Pool::builder()
            .max_size(16)
            .connection_timeout(Duration::from_millis(500))
            .build(manager)
            .expect("fail to create connection pool to mysql");
        MysqlClient {
            pool
        }
    }
}