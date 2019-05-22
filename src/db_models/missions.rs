use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid)]
#[table_name = "emtm_missions"]
pub struct Mission {
    pub mid: i32,
    pub cow_uid: i32,
    pub bounty: i32,
    pub risk: i32,
    pub name: String,
    pub content: String,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "emtm_missions"]
pub struct NewMission<'a> {
    pub cow_uid: i32,
    pub bounty: i32,
    pub name: &'a str,
    pub content: &'a str,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
}

pub const STATE_ACCEPT: u8 = 0;
pub const STATE_FINISHED: u8 = 1;
pub const STATE_CANCELLED: u8 = 2;

#[derive(Queryable, Insertable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid, student_uid)]
#[table_name = "emtm_participants"]
pub struct Participant {
    pub mid: i32,
    pub student_uid: i32,
    pub state: u8,
}
