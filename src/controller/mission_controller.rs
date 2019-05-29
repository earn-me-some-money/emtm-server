use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

use crate::search;

pub trait MissionController {
    /// Add a mission, returns an error if failed
    /// # Arguments
    /// * 'mission' - a mission, the mid field and participant field will be ignored
    fn add_mission(&self, mission: &models::missions::Mission) -> Result<(), DbError>;
    /// Given a mission, update it using the mid field
    /// all fields other than mid and participants will be updated, make sure you only change those
    /// fields that need to be updated
    /// To update participants, use add_participants and update_participant
    /// # Arguments
    /// * 'mission' - a mission, the mid field will be used to find the mission to be updated
    fn update_mission(&self, mission: &models::missions::Mission) -> Result<(), DbError>;
    /// Add a participant to the corresponding mission
    /// the function will fail if any of the participants failed to insert
    /// # Arguments
    /// * 'mid' - the id of the mission to add a participant
    /// * 'parts' - the info of the new participant
    fn add_participants(
        &self,
        mid: i32,
        part: &[models::missions::Participant],
    ) -> Result<(), DbError>;
    /// Update the state of a participant
    /// the mid and student_uid will be used to find the record
    /// # Arguments
    /// * 'parts' - the participants info
    fn update_participant(
        &self,
        mid: i32,
        part: &models::missions::Participant,
    ) -> Result<(), DbError>;
    /// Helper function to get all the mission's participants
    /// # Arguments
    /// * 'mid' - the id of the mission
    fn get_mission_participants(&self, mid: i32) -> Vec<db_models::missions::Participant>;
    /// Get all the mission posted by a poster
    /// # Arguments
    /// * 'poster_uid' - the id of the poster
    fn get_poster_missions(&self, poster_uid: i32) -> Vec<models::missions::Mission>;
    /// Get a particular the mission posted by a poster
    /// # Arguments
    /// * 'poster_uid' - the id of the poster
    /// * 'name' - The name of the mission
    fn get_posted_name_missions(
        &self,
        poster_uid: i32,
        name: &str,
    ) -> Option<models::missions::Mission>;
    /// Get all the mission joined by a student (not posted by!)
    /// # Arguments
    /// * 'student_id' - the id of the student
    fn get_student_missions(&self, student_id: i32) -> Vec<models::missions::Mission>;
    /// Get all the missions in a list, participants will not be loaded to save time,
    /// use get_mission_from_mid to get the complete mission info!
    fn get_missions_list(&self) -> Vec<models::missions::Mission>;
    /// Get mission with mid
    /// # Arguments
    /// * 'mid' - the id of the mission
    fn get_mission_from_mid(&self, mid: i32) -> Option<models::missions::Mission>;
}

impl MissionController for Controller {
    fn add_mission(&self, mission: &models::missions::Mission) -> Result<(), DbError> {
        use crate::schema::emtm_missions;
        use db_models::missions::*;
        let db_mission = mission.to_db().0;
        let new_mission = NewMission::from_mission(&db_mission);

        let result = diesel::insert_into(emtm_missions::table)
            .values(&new_mission)
            .execute(&self.connection);

        match result {
            Ok(_) => {
                let added_mission =
                    self.get_posted_name_missions(mission.poster_uid, &mission.name);
                match added_mission {
                    Some(new_mission) => {
                        info!("Added one mission with mid {}", new_mission.mid);
                        search::add_mission(&new_mission);
                        Ok(())
                    }
                    None => {
                        warn!(
                            "Failed to add mission with name {}: Unknown reason",
                            mission.name
                        );
                        Err(DbError::new("unknown error"))
                    }
                }
            }
            Err(error) => {
                info!(
                    "Failed to add mission with name {}: {}",
                    mission.name, error
                );
                Err(DbError::new(&error.to_string()))
            }
        }
    }

    fn update_mission(&self, mission: &models::missions::Mission) -> Result<(), DbError> {
        let db_mission = mission.to_db().0;
        let update_res = diesel::update(&db_mission)
            .set(&db_mission)
            .execute(&self.connection);

        match update_res {
            Ok(row) => {
                search::delete_mission(mission.mid);
                search::add_mission(mission);
                info!("Updated {} mission with mid {}", row, db_mission.mid);
                Ok(())
            }
            Err(e) => {
                warn!(
                    "Failed to update mission with mid {}: {}",
                    db_mission.mid, e
                );
                Err(DbError::new(&e.to_string()))
            }
        }
    }

    fn add_participants(
        &self,
        mid: i32,
        part: &[models::missions::Participant],
    ) -> Result<(), DbError> {
        use crate::schema::emtm_participants;
        use db_models::missions::Participant;
        let db_parts: Vec<Participant> = part.iter().map(|p| p.to_db(mid)).collect();

        let result = diesel::insert_into(emtm_participants::table)
            .values(&db_parts)
            .execute(&self.connection);

        match result {
            Ok(num) => {
                info!("Inserted {} participants", num);
                Ok(())
            }
            Err(error) => {
                info!("Failed to add paticipants: {}", error);
                Err(DbError::new(&error.to_string()))
            }
        }
    }

    fn update_participant(
        &self,
        mid: i32,
        part: &models::missions::Participant,
    ) -> Result<(), DbError> {
        let db_part = part.to_db(mid);
        let update_res = diesel::update(&db_part)
            .set(&db_part)
            .execute(&self.connection);

        match update_res {
            Ok(row) => {
                info!(
                    "Updated {} participants with mid {} and uid {}",
                    row, db_part.mid, db_part.student_uid
                );
                Ok(())
            }
            Err(e) => {
                warn!("Failed to update paticipant: {}", e);
                Err(DbError::new(&e.to_string()))
            }
        }
    }

    fn get_mission_participants(&self, mid_: i32) -> Vec<db_models::missions::Participant> {
        use crate::schema::emtm_participants::dsl::*;
        use db_models::missions::*;
        let result = emtm_participants
            .filter(mid.eq(mid_))
            .load::<Participant>(&self.connection);

        match result {
            Ok(parts) => parts,
            Err(error) => {
                error!(
                    "Panic when querying participants of mission {}: {}",
                    mid_, error
                );
                panic!(error.to_string());
            }
        }
    }

    fn get_poster_missions(&self, poster_uid_: i32) -> Vec<models::missions::Mission> {
        use crate::schema::emtm_missions::dsl::*;
        use db_models::missions::*;
        let result = emtm_missions
            .filter(poster_uid.eq(poster_uid_))
            .load::<Mission>(&self.connection);
        match result {
            Ok(missions) => {
                let mission_list = missions
                    .into_iter()
                    .map(|m| {
                        let parts = self.get_mission_participants(m.mid);
                        models::missions::Mission::from_db(m, parts)
                    })
                    .collect();

                mission_list
            }
            Err(e) => {
                error!(
                    "Panic when finding mission for poster {}: {}",
                    poster_uid_, e
                );
                panic!(e.to_string());
            }
        }
    }

    fn get_posted_name_missions(
        &self,
        poster_uid_: i32,
        name_: &str,
    ) -> Option<models::missions::Mission> {
        use crate::schema::emtm_missions::dsl::*;
        use db_models::missions::*;
        let result = emtm_missions
            .filter(poster_uid.eq(poster_uid_))
            .filter(name.eq(name_))
            .load::<Mission>(&self.connection);
        match result {
            Ok(mut missions) => {
                if missions.is_empty() {
                    None
                } else {
                    let mission = missions.remove(0);
                    let parts = self.get_mission_participants(mission.mid);
                    Some(models::missions::Mission::from_db(mission, parts))
                }
            }
            Err(e) => {
                error!(
                    "Panic when finding mission for poster {}: {}",
                    poster_uid_, e
                );
                panic!(e.to_string());
            }
        }
    }

    fn get_student_missions(&self, student_id: i32) -> Vec<models::missions::Mission> {
        use crate::schema::emtm_participants::dsl::*;
        use db_models::missions::Participant;
        let part_res = emtm_participants
            .filter(student_uid.eq(student_id))
            .load::<Participant>(&self.connection);

        match part_res {
            Ok(parts) => {
                let mission_list = parts
                    .into_iter()
                    .filter_map(|p| self.get_mission_from_mid(p.mid))
                    .collect();

                mission_list
            }
            Err(e) => {
                error!(
                    "Panic when finding mission for student {}: {}",
                    student_id, e
                );
                panic!(e.to_string());
            }
        }
    }

    fn get_missions_list(&self) -> Vec<models::missions::Mission> {
        use crate::schema::emtm_missions::dsl::*;
        use db_models::missions::*;
        let result = emtm_missions.load::<Mission>(&self.connection);

        match result {
            Ok(missions) => missions
                .into_iter()
                .map(|m| models::missions::Mission::from_db(m, vec![]))
                .collect(),
            Err(e) => {
                error!("Panic when loading mission list: {}", e);
                panic!(e.to_string());
            }
        }
    }

    fn get_mission_from_mid(&self, mid_: i32) -> Option<models::missions::Mission> {
        use crate::schema::emtm_missions::dsl::*;
        use db_models::missions::*;
        let result = emtm_missions
            .filter(mid.eq(mid_))
            .load::<Mission>(&self.connection);
        match result {
            Ok(mut missions) => {
                if missions.is_empty() {
                    None
                } else {
                    let m = missions.swap_remove(0);
                    let parts = self.get_mission_participants(m.mid);
                    Some(models::missions::Mission::from_db(m, parts))
                }
            }
            Err(e) => {
                error!("Panic when finding mission with mid {}: {}", mid_, e);
                panic!(e.to_string());
            }
        }
    }
}
