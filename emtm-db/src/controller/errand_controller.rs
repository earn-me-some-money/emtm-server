use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

pub trait ErrandController {
    /// Adds a errand information
    /// # Arguments
    /// * 'errand_item' - The errand instance to be added
    fn add_errand(&self, errand_item: &models::Errand) -> Result<(), DbError>;
    /// Query a errand from mid
    /// # Arguments
    /// 'mid' - The mid of the errand to be retrieved
    fn get_errand(&self, mid: i32) -> Option<models::Errand>;
}

impl ErrandController for Controller {
    fn add_errand(&self, errand_item: &models::Errand) -> Result<(), DbError> {
        use crate::schema::emtm_errands;
        let result = diesel::insert_into(emtm_errands::table)
            .values(errand_item)
            .execute(&self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("Failed to insert errand: {}", err);
                Err(DbError::new(&err.to_string()))
            }
        }
    }

    fn get_errand(&self, mid_: i32) -> Option<models::Errand> {
        use crate::schema::emtm_errands::dsl::*;
        let result = emtm_errands
            .filter(mid.eq(mid_))
            .load::<db_models::Errand>(&self.connection);

        let mut errands = result.unwrap_or_else(|err| {
            error!("Panic when finding errand by mid {}: {}", mid_, err);
            panic!(err.to_string());
        });
        if errands.is_empty() {
            None
        } else {
            //Get first element
            Some(errands.swap_remove(0))
        }
    }
}
