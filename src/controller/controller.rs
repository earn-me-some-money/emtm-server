use diesel::mysql::MysqlConnection;

use crate::controller::connection;


pub struct Controller {
    pub connection: MysqlConnection
}


impl Controller {
    pub fn new() -> Self {
        Self {
            connection: connection::establish()
        }
    }


}