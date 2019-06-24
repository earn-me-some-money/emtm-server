use crate::schema::*;
use chrono::NaiveDateTime;

pub mod mission_type {
    pub const QUESTIONNAIRE: i8 = 0;
    pub const TRANSLATION: i8 = 1;
    pub const ERRANDS: i8 = 2;
    pub const TRADE: i8 = 3;
}

pub mod part_state_type {
    pub const STATE_ACCEPT: i8 = 0;
    pub const STATE_FINISHED: i8 = 1;
    pub const STATE_CANCELLED: i8 = 2;
}

#[derive(Queryable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid)]
#[table_name = "emtm_missions"]
pub struct Mission {
    pub mid: i32,
    pub poster_uid: i32,
    pub bounty: i32,
    pub risk: i32,
    pub name: String,
    pub mission_type: i8,
    pub content: String,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub max_participants: Option<i32>,
    pub min_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub school: Option<i32>,
    pub min_finished: Option<i32>,
    pub min_credit: Option<i32>,
    pub major: Option<String>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "emtm_missions"]
pub struct NewMission<'a> {
    pub poster_uid: i32,
    pub bounty: i32,
    pub risk: i32,
    pub name: &'a str,
    pub mission_type: i8,
    pub content: &'a str,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub max_participants: Option<i32>,
    pub min_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub school: Option<i32>,
    pub min_finished: Option<i32>,
    pub min_credit: Option<i32>,
    pub major: Option<&'a str>,
}

impl<'a> NewMission<'a> {
    pub fn from_mission(mission: &'a Mission) -> Self {
        Self {
            poster_uid: mission.poster_uid,
            bounty: mission.bounty,
            risk: mission.risk,
            name: &mission.name,
            mission_type: mission.mission_type,
            content: &mission.content,
            post_time: mission.post_time,
            deadline: mission.deadline,
            max_participants: mission.max_participants,
            min_grade: mission.min_grade,
            max_grade: mission.max_grade,
            school: mission.school,
            min_finished: mission.min_finished,
            min_credit: mission.min_credit,
            major: mission.major.as_ref().map(String::as_str),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Clone, AsChangeset, Identifiable)]
#[primary_key(mid, student_uid)]
#[table_name = "emtm_participants"]
pub struct Participant {
    pub mid: i32,
    pub student_uid: i32,
    pub state: i8,
}
