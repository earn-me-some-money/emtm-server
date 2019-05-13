use diesel::mysql::MysqlConnection;

use crate::controller::connection;

/// The database controller for CRUD in emtm's database.
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