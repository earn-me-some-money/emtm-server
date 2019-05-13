use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let migration_result = diesel_migrations::run_pending_migrations(&conn);

    match migration_result {
        Ok(_) => {},
        Err(error) => {
            error!("Error when running migrations: {}", error);
            panic!(error);
        },
    }
    conn
}
