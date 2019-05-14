use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn establish(url: &str) -> MysqlConnection {
    MysqlConnection::establish(url).expect(&format!("Error connecting to {}", url))
}
