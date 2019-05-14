use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

pub trait UserController {
    //Add Users
    /// Add cows with a vector of Cows, returns the uid of added cows
    /// # Arguments
    /// * 'cows' - A vector containing all the cow instances to be added
    fn add_cows(&self, cows: &Vec<models::users::Cow>) -> Vec<Result<(i32), DbError>>;
    /// Add students with a vector of Students, returns the uid of added students
    /// # Arguments
    /// * 'students' - A vector containing all the student instances to be added
    fn add_students(&self, students: &Vec<models::users::Student>) -> Vec<Result<(i32), DbError>>;
    /// Helper function for add_cows and add_db_user, returns the added user
    /// # Arguments
    /// * 'new_user' - A vector containing the user to be added
    fn add_db_user(
        &self,
        new_user: db_models::users::NewUser,
    ) -> Result<db_models::users::User, DbError>;

    // Update Users
    /// Update users according to the uid fields of the given user enum instances.
    /// all fields except for the uid fields will be updated.
    /// # Arguments
    /// * 'updated_users' - The list of users to be updated
    fn update_users(&self, updated_users: &Vec<models::users::User>) -> Vec<Result<(), DbError>>;
    /// Helper functions, update db user
    fn update_db_user(&self, db_user: db_models::users::User) -> Result<(), DbError>;
    /// Helper functions, update db cow
    fn update_db_cow(&self, db_cow: db_models::users::Cow) -> Result<(), DbError>;
    /// Helper functions, update db student
    fn update_db_student(&self, db_student: db_models::users::Student) -> Result<(), DbError>;

    //Query Users
    /// Query a user from a given user identifier, returns None if user doesn't exist
    /// # Arguments
    /// * 'to_search' - The unique user identifier to be searched
    fn get_user_from_identifier(
        &self,
        to_search: models::users::UserId,
    ) -> Option<models::users::User>;
    /// Helper function, returns a user enum from a given db user
    /// # Arguments
    /// * 'u' - The user to find a user
    fn get_user_from_db_user(&self, u: db_models::users::User) -> Option<models::users::User>;
    /// Query a cow from a given uid
    /// # Arguments
    /// * 'to_search' - The uid to be searched
    fn get_db_cow_from_uid(&self, to_search: i32) -> Option<db_models::users::Cow>;
    /// Query a cow from a given uid
    /// # Arguments
    /// * 'to_search' - The uid to be searched
    fn get_db_student_from_uid(&self, to_search: i32) -> Option<db_models::users::Student>;
    /// Get the user type of a user from uid
    /// # Arguments
    /// * 'to_search' - The uid to be searched
    fn get_user_type_from_uid(&self, to_search: i32) -> Option<i8>;
    /// Get a student instance from school id and student id
    /// # Arguments
    /// * 'school_id' - The school id of the student's university
    /// * 'student_id' - The student id of the student.
    fn get_student_uid_from_school_info(&self, school_id: i32, student_id: &str) -> Option<i32>;
}

impl UserController for Controller {
    fn add_cows(&self, cows: &Vec<models::users::Cow>) -> Vec<Result<(i32), DbError>> {
        let mut u_results = vec![];

        // insert into user table
        for cow in cows {
            info!("Adding cow with username {}", cow.username);
            let db_u = db_models::users::NewUser {
                wechat_id: &cow.wechat_id,
                phone: &cow.phone,
                personal_info: &cow.personal_info,
                email: &cow.email,
                username: &cow.username,
                verified: cow.verified,
                tokens: cow.tokens,
                user_type: 0,
            };
            u_results.push(self.add_db_user(db_u))
        }

        // insert into cow table
        let mut results = vec![];
        for (cow, u_res) in cows.iter().zip(u_results.iter()) {
            if let Ok(user) = u_res {
                let new_cow = db_models::users::NewCow {
                    uid: user.uid,
                    company: &cow.company,
                };
                use crate::schema::emtm_cows;
                let res = diesel::insert_into(emtm_cows::table)
                    .values(&new_cow)
                    .execute(&self.connection);
                results.push(match res {
                    Ok(_) => {
                        let db_cow = self.get_db_cow_from_uid(user.uid);
                        match db_cow {
                            Some(_cow) => {
                                info!("Successfully added cow with uid {}", user.uid);
                                Ok(user.uid)
                            }
                            None => {
                                warn!(
                                    "Failed to add cow with username {}, unknown error",
                                    cow.username
                                );
                                Err(DbError::new("unknown error"))
                            }
                        }
                    }
                    Err(error) => {
                        info!("Failed to add cow with uid {}: {}", user.uid, error);
                        Err(DbError::new(&error.to_string()))
                    }
                });
            } else if let Err(error) = u_res {
                results.push(Err(DbError::new(&error.to_string())))
            }
        }

        results
    }

    fn add_students(&self, students: &Vec<models::users::Student>) -> Vec<Result<(i32), DbError>> {
        let mut u_results = vec![];

        for student in students {
            info!("Adding student with username {}", student.username);
            let db_u = db_models::users::NewUser {
                wechat_id: &student.wechat_id,
                phone: &student.phone,
                personal_info: &student.personal_info,
                email: &student.email,
                username: &student.username,
                verified: student.verified,
                tokens: student.tokens,
                user_type: 1,
            };
            u_results.push(self.add_db_user(db_u))
        }

        let mut results = vec![];
        for (student, u_res) in students.iter().zip(u_results.iter()) {
            if let Ok(user) = u_res {
                let new_student = db_models::users::NewStudent {
                    uid: user.uid,
                    school_id: student.school_id,
                    student_id: &student.student_id,
                    credit: student.credit,
                    accepted: student.accepted,
                    finished: student.finished,
                    major: &student.major,
                    year: student.year,
                };
                use crate::schema::emtm_students;
                let res = diesel::insert_into(emtm_students::table)
                    .values(&new_student)
                    .execute(&self.connection);
                results.push(match res {
                    Ok(_) => {
                        let db_student = self.get_db_student_from_uid(user.uid);
                        match db_student {
                            Some(_student) => {
                                info!("Successfully added student with uid {}", user.uid);
                                Ok(user.uid)
                            }
                            None => {
                                warn!(
                                    "Failed to add student with username {}, unknown error",
                                    student.username
                                );
                                Err(DbError::new("unknown error"))
                            }
                        }
                    }
                    Err(error) => {
                        info!("Failed to add student with uid {}: {}", user.uid, error);
                        Err(DbError::new(&error.to_string()))
                    }
                });
            } else if let Err(error) = u_res {
                results.push(Err(DbError::new(&error.to_string())))
            }
        }

        results
    }

    fn add_db_user(
        &self,
        new_user: db_models::users::NewUser,
    ) -> Result<db_models::users::User, DbError> {
        use crate::schema::emtm_users;
        let result = diesel::insert_into(emtm_users::table)
            .values(&new_user)
            .execute(&self.connection);

        match result {
            Ok(_) => {
                let mut user = {
                    use crate::schema::emtm_users::dsl::*;
                    emtm_users
                        .filter(wechat_id.eq(&new_user.wechat_id))
                        .load::<db_models::users::User>(&self.connection)
                        .expect("database error")
                };
                if !user.is_empty() {
                    info!("Successfully added user with uid {}", user[0].uid);
                    Ok(user.remove(0))
                } else {
                    warn!(
                        "Failed to add user with wechat id {}, unknown error",
                        new_user.wechat_id
                    );
                    Err(DbError::new("unknown error"))
                }
            }
            Err(error) => {
                info!(
                    "Failed to add user with username {}: {}",
                    new_user.username, error
                );
                Err(DbError::new(&error.to_string()))
            }
        }
    }

    fn update_users(&self, updated_users: &Vec<models::users::User>) -> Vec<Result<(), DbError>> {
        let mut results = vec![];
        use models::users::User::*;
        for user in updated_users {
            match user {
                Student(s) => {
                    // Check type first.
                    let u_type = self.get_user_type_from_uid(s.uid);
                    if let Some(ty) = u_type {
                        if ty != db_models::users::TYPE_STUDENT {
                            results.push(Err(DbError::new(&format!(
                                "cannot change type of uid {} to student",
                                s.uid
                            ))));
                            continue;
                        }
                    } else {
                        results.push(Err(DbError::new(&format!(
                            "cannot find user with uid {}",
                            s.uid
                        ))));
                        continue;
                    }
                    // Update user and student table
                    let (db_u, db_s) = s.to_db();
                    let u_res = self.update_db_user(db_u);
                    if let Ok(_) = u_res {
                        let s_res = self.update_db_student(db_s);
                        results.push(s_res);
                    } else {
                        results.push(u_res);
                    }
                }
                Cow(c) => {
                    let u_type = self.get_user_type_from_uid(c.uid);
                    if let Some(ty) = u_type {
                        if ty != db_models::users::TYPE_COW {
                            results.push(Err(DbError::new(&format!(
                                "cannot change type of uid {} to cow",
                                c.uid
                            ))));
                            continue;
                        }
                    } else {
                        results.push(Err(DbError::new(&format!(
                            "cannot find user with uid {}",
                            c.uid
                        ))));
                        continue;
                    }
                    let (db_u, db_c) = c.to_db();
                    let u_res = self.update_db_user(db_u);
                    if let Ok(_) = u_res {
                        let c_res = self.update_db_cow(db_c);
                        results.push(c_res);
                    } else {
                        results.push(u_res);
                    }
                }
            }
        }
        results
    }

    fn update_db_user(&self, db_user: db_models::users::User) -> Result<(), DbError> {
        // UPDATE `emtm_users` SET [...]
        // WHERE `emtm_users`.`uid` = db_user.uid
        let update_res = diesel::update(&db_user)
            .set(&db_user)
            .execute(&self.connection);

        match update_res {
            Ok(row) => {
                info!("Updated {} user with uid {}", row, db_user.uid);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to update user with uid {}: {}", db_user.uid, e);
                Err(DbError::new(&e.to_string()))
            }
        }
    }

    fn update_db_cow(&self, db_cow: db_models::users::Cow) -> Result<(), DbError> {
        let update_res = diesel::update(&db_cow)
            .set(&db_cow)
            .execute(&self.connection);
        match update_res {
            Ok(row) => {
                info!("Updated {} cow with uid {}", row, db_cow.uid);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to update cow with uid {}: {}", db_cow.uid, e);
                Err(DbError::new(&e.to_string()))
            }
        }
    }

    fn update_db_student(&self, db_student: db_models::users::Student) -> Result<(), DbError> {
        let update_res = diesel::update(&db_student)
            .set(&db_student)
            .execute(&self.connection);
        match update_res {
            Ok(row) => {
                info!("Updated {} student with uid {}", row, db_student.uid);
                Ok(())
            }
            Err(e) => {
                warn!(
                    "Failed to update student with uid {}: {}",
                    db_student.uid, e
                );
                Err(DbError::new(&e.to_string()))
            }
        }
    }

    fn get_user_from_identifier(
        &self,
        to_search: models::users::UserId,
    ) -> Option<models::users::User> {
        use crate::schema::emtm_users::dsl::*;
        use db_models::users::*;
        use models::users::UserId;

        let table = emtm_users;

        let query_result = {
            match to_search {
                UserId::Uid(id) => table.filter(uid.eq(id)).load::<User>(&self.connection),
                UserId::WechatId(id) => table
                    .filter(wechat_id.eq(id))
                    .load::<User>(&self.connection),
                UserId::Phone(p) => table
                    .filter(phone.eq(p))
                    .load::<User>(&self.connection),
                UserId::Email(em) => table
                    .filter(email.eq(em))
                    .load::<User>(&self.connection),
                UserId::SchoolInfo(school_id, student_id) => {
                    match self.get_student_uid_from_school_info(school_id, student_id) {
                        Some(id) => table
                            .filter(uid.eq(id))
                            .load::<User>(&self.connection),
                        None => {
                            return None;
                        }
                    }
                }
            }
        };

        let db_u = {
            match query_result {
                Ok(mut users) => {
                    if users.is_empty() {
                        return None;
                    } else {
                        //Get first element
                        users.swap_remove(0)
                    }
                }
                Err(error) => {
                    error!(
                        "Panic when querying user with uid {:?}: {}",
                        to_search, error
                    );
                    panic!(error.to_string());
                }
            }
        };

        self.get_user_from_db_user(db_u)
    }

    fn get_user_from_db_user(&self, db_u: db_models::users::User) -> Option<models::users::User> {
        use models::users::User;

        match db_u.user_type {
            db_models::users::TYPE_COW => {
                let db_c = self.get_db_cow_from_uid(db_u.uid);
                match db_c {
                    Some(c) => Some(User::Cow(models::users::Cow::from_db(db_u, c))),
                    None => {
                        warn!("Cow user with uid {} can't be found in cow table", db_u.uid);
                        None
                    }
                }
            }
            db_models::users::TYPE_STUDENT => {
                let db_s = self.get_db_student_from_uid(db_u.uid);
                match db_s {
                    Some(s) => Some(User::Student(models::users::Student::from_db(db_u, s))),
                    None => {
                        warn!("Cow user with uid {} can't be found in cow table", db_u.uid);
                        None
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected user type {} when querying uid {}",
                    db_u.user_type, db_u.uid
                );
                None
            }
        }
    }

    fn get_db_cow_from_uid(&self, to_search: i32) -> Option<db_models::users::Cow> {
        use crate::schema::emtm_cows::dsl::*;
        use db_models::users::*;

        let results = emtm_cows
            .filter(uid.eq(to_search))
            .load::<Cow>(&self.connection);

        match results {
            Ok(mut cows) => {
                if cows.is_empty() {
                    None
                } else {
                    //Get first element
                    Some(cows.swap_remove(0))
                }
            }
            Err(error) => {
                error!("Panic when querying cow with uid {}: {}", to_search, error);
                panic!(error.to_string());
            }
        }
    }

    fn get_db_student_from_uid(&self, to_search: i32) -> Option<db_models::users::Student> {
        use crate::schema::emtm_students::dsl::*;
        use db_models::users::*;

        let results = emtm_students
            .filter(uid.eq(to_search))
            .load::<Student>(&self.connection);

        match results {
            Ok(mut students) => {
                if students.is_empty() {
                    None
                } else {
                    //Get first element
                    Some(students.swap_remove(0))
                }
            }
            Err(error) => {
                error!(
                    "Panic when querying student with uid {}: {}",
                    to_search, error
                );
                panic!(error.to_string());
            }
        }
    }

    fn get_user_type_from_uid(&self, to_search: i32) -> Option<i8> {
        use crate::schema::emtm_users::dsl::*;

        let results = emtm_users
            .filter(uid.eq(to_search))
            .select(user_type)
            .load::<i8>(&self.connection);

        match results {
            Ok(mut types) => {
                if types.is_empty() {
                    None
                } else {
                    //Get first element
                    Some(types.swap_remove(0))
                }
            }
            Err(error) => {
                error!(
                    "Panic when querying user type with uid {}: {}",
                    to_search, error
                );
                panic!(error.to_string());
            }
        }
    }

    fn get_student_uid_from_school_info(&self, school_id_: i32, student_id_: &str) -> Option<i32> {
        use crate::schema::emtm_students::dsl::*;

        let results = emtm_students
            .filter(school_id.eq(school_id_))
            .filter(student_id.eq(student_id_))
            .select(uid)
            .load::<i32>(&self.connection);

        match results {
            Ok(mut students) => {
                if students.is_empty() {
                    None
                } else {
                    //Get first element
                    Some(students.swap_remove(0))
                }
            }
            Err(error) => {
                error!("Panic when querying student with school info: {}", error);
                panic!(error.to_string());
            }
        }
    }
}
