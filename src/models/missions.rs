use crate::db_models;
use chrono::NaiveDateTime;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
pub enum PartState {
    Accepted,
    Finished,
    Cancelled,
}

impl PartState {
    /// Construct PartState from database value
    pub fn from_val(val: i8) -> Self {
        match val {
            db_models::missions::part_state_type::STATE_ACCEPT => PartState::Accepted,
            db_models::missions::part_state_type::STATE_FINISHED => PartState::Finished,
            db_models::missions::part_state_type::STATE_CANCELLED => PartState::Cancelled,

            _ => panic!("Unexpected state value"),
        }
    }
    /// get the database value from a PartState
    pub fn to_val(&self) -> i8 {
        match self {
            PartState::Accepted => db_models::missions::part_state_type::STATE_ACCEPT,
            PartState::Finished => db_models::missions::part_state_type::STATE_FINISHED,
            PartState::Cancelled => db_models::missions::part_state_type::STATE_CANCELLED,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
pub enum MissionType {
    Questionnaire,
    Translation,
    Errands,
}

impl MissionType {
    /// Construct MissionType from database value
    pub fn from_val(val: i8) -> Self {
        match val {
            db_models::missions::mission_type::QUESTIONNAIRE => MissionType::Questionnaire,
            db_models::missions::mission_type::TRANSLATION => MissionType::Translation,
            db_models::missions::mission_type::ERRANDS => MissionType::Errands,
            _ => panic!("Unexpected state value"),
        }
    }
    /// get the database value from a MissionType
    pub fn to_val(&self) -> i8 {
        match self {
            MissionType::Questionnaire => db_models::missions::mission_type::QUESTIONNAIRE,
            MissionType::Translation => db_models::missions::mission_type::TRANSLATION,
            MissionType::Errands => db_models::missions::mission_type::ERRANDS,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mission {
    pub mid: i32,
    pub poster_uid: i32,
    pub bounty: i32,
    pub risk: i32,
    pub name: String,
    pub mission_type: MissionType,
    pub content: String,
    pub post_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub participants: Vec<Participant>,
    pub max_participants: i32,
    //Grade restriction: [min, max]
    pub min_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub school: Option<i32>,
    pub min_finished: Option<i32>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
            poster_uid: mission.poster_uid,
            bounty: mission.bounty,
            risk: mission.risk,
            name: mission.name,
            mission_type: MissionType::from_val(mission.mission_type),
            content: mission.content,
            post_time: mission.post_time,
            deadline: mission.deadline,
            participants: parts
                .into_iter()
                .map(|part| Participant::from_db(part))
                .collect(),
            max_participants: mission.max_participants,
            min_grade: mission.min_grade,
            max_grade: mission.max_grade,
            school: mission.school,
            min_finished: mission.min_finished
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
                poster_uid: self.poster_uid,
                bounty: self.bounty,
                risk: self.risk,
                name: self.name.clone(),
                mission_type: self.mission_type.to_val(),
                content: self.content.clone(),
                post_time: self.post_time,
                deadline: self.deadline,
                max_participants: self.max_participants,
                min_grade: self.min_grade,
                max_grade: self.max_grade,
                school: self.school,
                min_finished: self.min_finished
            },
            self.participants
                .iter()
                .map(|part| part.to_db(self.mid))
                .collect(),
        )
    }
}
