use crate::schema::*;

#[derive(Insertable, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid)]
#[table_name = "emtm_errands"]
pub struct Errand {
    pub mid: i32,
    pub pickup_address: String,
    pub phone_number: String,
    pub item_code: Option<String>,
    pub deliver_address: String,
    pub other_info: String,
}
