use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

pub trait TradeController {
    /// Adds a trade information
    /// # Arguments
    /// * 'trade_item' - The trade instance to be added
    fn add_trade(&self, trade_item: &models::Trade) -> Result<(), DbError>;
    /// Query a trade from mid
    /// # Arguments
    /// 'mid' - The mid of the trade to be retrieved
    fn get_trade(&self, mid: i32) -> Option<models::Trade>;
}

impl TradeController for Controller {
    fn add_trade(&self, trade_item: &models::Trade) -> Result<(), DbError> {
        use crate::schema::emtm_trades;
        let result = diesel::insert_into(emtm_trades::table)
            .values(trade_item)
            .execute(&self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("Failed to insert answer: {}", err);
                Err(DbError::new(&err.to_string()))
            }
        }
    }

    fn get_trade(&self, mid_: i32) -> Option<models::Trade> {
        use crate::schema::emtm_trades::dsl::*;
        let result = emtm_trades
            .filter(mid.eq(mid_))
            .load::<db_models::Trade>(&self.connection);

        let mut trades = result.unwrap_or_else(|err| {
            error!("Panic when finding trade by mid {}: {}", mid_, err);
            panic!(err.to_string());
        });
        if trades.is_empty() {
            None
        } else {
            //Get first element
            Some(trades.swap_remove(0))
        }
    }
}
