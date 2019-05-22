use crate::db_models;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Eq, Hash, Clone)]
pub enum PartState {
    Accepted,
    Finished,
    Cancelled,
}

impl PartState {
    /// Construct PartState from database value
    pub fn from_val(val: u8) -> Self {
        match val {
            db_models::missions::STATE_ACCEPT => PartState::Accepted,
            db_models::missions::STATE_FINISHED => PartState::Finished,
            db_models::missions::STATE_CANCELLED => PartState::Cancelled,

            _ => panic!("Unexpected state value"),
        }
    }

    /// get the database value from a PartState
    pub fn to_val(&self) -> u8 {
        match self {
            PartState::Accepted => db_models::missions::STATE_ACCEPT,
            PartState::Finished => db_models::missions::STATE_FINISHED,
            PartState::Cancelled => db_models::missions::STATE_CANCELLED,
        }
    }
}

pub struct Mission {
    pub mid: i32,
    pub cow_uid: i32,
    pub bounty: i32,
    pub risk: i32,
    pub name: String,
    pub content: String,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub participants: Vec<Participant>,
}

pub struct Participant {
    pub student_uid: i32,
    pub state: PartState,
}

impl Participant {
    pub fn from_db(part: db_models::missions::Participant) -> Self {
        Self {
            student_uid: part.student_uid,
            state: PartState::from_val(part.state),
        }
    }

    pub fn to_db(&self, mid: i32) -> db_models::missions::Participant {
        db_models::missions::Participant {
            mid,
            student_uid: self.student_uid,
            state: self.state.to_val(),
        }
    }
}

impl Mission {
    pub fn from_db(
        mission: db_models::missions::Mission,
        parts: Vec<db_models::missions::Participant>,
    ) -> Self {
        Self {
            mid: mission.mid,
            cow_uid: mission.cow_uid,
            bounty: mission.bounty,
            risk: mission.risk,
            name: mission.name,
            content: mission.content,
            post_time: mission.post_time,
            deadline: mission.deadline,
            participants: parts
                .into_iter()
                .map(|part| Participant::from_db(part))
                .collect(),
        }
    }

    pub fn to_db(
        &self,
    ) -> (
        db_models::missions::Mission,
        Vec<db_models::missions::Participant>,
    ) {
        (
            db_models::missions::Mission {
                mid: self.mid,
                cow_uid: self.cow_uid,
                bounty: self.bounty,
                risk: self.risk,
                name: self.name.clone(),
                content: self.content.clone(),
                post_time: self.post_time,
                deadline: self.deadline,
            },
            self.participants
                .iter()
                .map(|part| part.to_db(self.mid))
                .collect(),
        )
    }
}
