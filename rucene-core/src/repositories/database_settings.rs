use std::rc::Rc;
use diesel::mysql::Mysql;
use crate::repositories::mysql_client::MysqlClient;

pub struct DatabaseSettings {
    pub mysql_client: Rc<MysqlClient>
}

impl DatabaseSettings {
    pub fn new(mysql_client: Rc<MysqlClient>) -> DatabaseSettings {
        DatabaseSettings {mysql_client}
    }
}