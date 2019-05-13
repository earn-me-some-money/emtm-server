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

    /// Run migrations, panic if migration fails.
    /// The migrations folder must present on the current or parent directory.
    pub fn migrate(&self) {
        let migration_result = diesel_migrations::run_pending_migrations(&self.connection);

        match migration_result {
            Ok(_) => {},
            Err(error) => {
                error!("Error when running migrations: {}", error);
                panic!(error);
            },
        }
    }

}