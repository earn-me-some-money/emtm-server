use crate::schema::*;

#[derive(Insertable, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid)]
#[table_name = "emtm_trades"]
pub struct Trade {
    pub mid: i32,
    pub item_type: String,
    pub item_info: String,
    pub item_condition: i8,
    pub address: String,
}



