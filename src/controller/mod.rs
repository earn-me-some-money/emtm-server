mod connection;
pub mod school_controller_zh;
pub mod user_controller;

pub use self::user_controller::UserController;
pub use self::school_controller_zh::SchoolControllerZh;

use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

/// The database controller for CRUD in emtm's database.
pub struct Controller {
    pub connection: MysqlConnection,
}

impl Controller {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            connection: connection::establish(&database_url),
        }
    }

    // Use the test database URL when testing.
    pub fn test_new() -> Self {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            connection: connection::establish(&database_url),
        }
    }

    /// Run migrations, panic if migration fails.
    /// The migrations folder must present on the current or parent directory.
    pub fn migrate(&self) {
        let migration_result = diesel_migrations::run_pending_migrations(&self.connection);
        match migration_result {
            Ok(_) => {}
            Err(error) => {
                error!("Error when running migrations: {}", error);
                panic!(error);
            }
        }
    }

    /// Revert all migrations
    pub fn revert_all(&self) {
        loop {
            let revert_res = diesel_migrations::revert_latest_migration(&self.connection);
            if let Err(_) = revert_res {
                break;
            }
        }
    }
}
