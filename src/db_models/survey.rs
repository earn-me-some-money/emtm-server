use crate::schema::*;


#[derive(Insertable, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid, ordering)]
#[table_name = "emtm_questions"]
pub struct Question {
    pub mid: i32,
    pub ordering: i32,
    pub description: String,
    pub choices: Vec<u8>
}


#[derive(Insertable, Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid, user_id)]
#[table_name = "emtm_answers"]
pub struct Answer {
    pub mid: i32,
    pub user_id: i32,
    pub user_answer: Vec<u8>
}


