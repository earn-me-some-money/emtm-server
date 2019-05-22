use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

pub trait MissionController {
    /// Given a mission, returns the mid of the mission after added to db
    /// # Arguments
    /// * 'mission' - a mission, the mid field will be ignored
    fn add_mission(mission: models::missions::Mission) -> Result<i32, DbError>;
    /// Given a mission, update it using the mid field
    /// all fields other than mid will be updated, make sure you only change those
    /// fields that need to be updated
    /// # Arguments
    /// * 'mission' - a mission, the mid field will be used to find the mission to be updated
    fn update_mission(mission: models::missions::Mission) -> Result<(), DbError>;
    /// Add a participant to the corresponding mission
    /// # Arguments
    /// * 'mid' - the id of the mission to add a participant
    /// * 'parts' - the info of the new participant
    fn add_participant(mid: i32, part: models::missions::Participant) -> Result<(), DbError>;
    /// Update the state of a participant
    /// the mid and student_uid will be used to find the record
    /// # Arguments
    /// * 'parts' - the participants info
    fn update_participant(part: models::missions::Participant) -> Result<(), DbError>;
}