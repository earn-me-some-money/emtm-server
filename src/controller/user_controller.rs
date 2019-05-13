use crate::controller::Controller;
use crate::db_models;
use crate::models;
use diesel::prelude::*;

pub trait UserController {

    //Add Users
    /// Add cows with a vector of Cows, returns the uid of added cows
    /// # Arguments
    /// * 'cows' - A vector containing all the cow instances to be added
    fn add_cows(&self, cows: Vec<models::users::Cow>) -> Vec<Result<(i32), String>>;
    /// Add students with a vector of Students, returns the uid of added students
    /// # Arguments
    /// * 'students' - A vector containing all the student instances to be added
    fn add_students(&self, students: Vec<models::users::Student>) -> Vec<Result<(i32), String>>;
    /// Helper function for add_cows and add_db_user, returns the added user
    /// # Arguments
    /// * 'new_user' - A vector containing the user to be added
    fn add_db_user(&self, new_user: db_models::users::NewUser) -> Result<db_models::users::User, String>;


    // Update Users


    //Query Users
    /// Query a user from a given uid, returns None if user doesn't exist
    /// # Arguments
    /// * 'to_search' - The uid to be search
    fn get_user_from_uid(&self, to_search: i32) -> Option<models::users::User>;
    /// Query a user from a given username
    /// # Arguments
    /// * 'name' - The username to be search
    fn get_user_from_username(&self, name: &str) -> Option<models::users::User>;
    /// Helper function, returns a user enum from a given db user
    /// # Arguments
    /// * 'u' - The user to find a user
    fn get_user_from_db_user(&self, u: db_models::users::User) -> Option<models::users::User>;
    /// Query a db user from a given username
    /// # Arguments
    /// * 'name' - The username to be search
    fn get_db_user_from_username(&self, name: &str) -> Option<db_models::users::User>;
    /// Query a cow from a given uid
    /// # Arguments
    /// * 'to_search' - The uid to be search
    fn get_cow_from_uid(&self, to_search: i32) -> Option<db_models::users::Cow>;
    /// Query a cow from a given uid
    /// # Arguments
    /// * 'to_search' - The uid to be search
    fn get_student_from_uid(&self, to_search: i32) -> Option<db_models::users::Student>;
}

impl UserController for Controller {

    fn add_cows(&self, cows: Vec<models::users::Cow>) -> Vec<Result<(i32), String>> {
        let mut u_results = vec![];

        // insert into user table
        for cow in &cows {
            info!("Adding cow with username {}", cow.username);
            let db_u = db_models::users::NewUser {
                wechat_id: &cow.wechat_id,
                phone: &cow.phone,
                personal_info: &cow.personal_info,
                email: &cow.email,
                username: &cow.username,
                verified: cow.verified,
                tokens: cow.tokens,
                user_type: 0
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
                        let db_cow = self.get_cow_from_uid(user.uid);
                        match db_cow {
                            Some(_cow) => {
                                info!("Successfully added cow with uid {}", user.uid);
                                Ok(user.uid)
                            }
                            None => {
                                warn!("Failed to add cow with username {}, unknown error", cow.username);
                                Err("unknown error".to_string())
                            }
                        }
                    }
                    Err(error) => {
                        info!("Failed to add cow with uid {}: {}", user.uid, error);
                        Err(error.to_string())
                    }
                });
            } else if let Err(error) = u_res {
                results.push(Err(error.to_string()))
            }
        }

        results
    }

    fn add_students(&self, students: Vec<models::users::Student>) -> Vec<Result<(i32), String>> {
        let mut u_results = vec![];

        for student in &students {
            info!("Adding student with username {}", student.username);
            let db_u = db_models::users::NewUser {
                wechat_id: &student.wechat_id,
                phone: &student.phone,
                personal_info: &student.personal_info,
                email: &student.email,
                username: &student.username,
                verified: student.verified,
                tokens: student.tokens,
                user_type: 1
            };
            u_results.push(self.add_db_user(db_u))
        }

        let mut results = vec![];
        for (student, u_res) in students.iter().zip(u_results.iter()) {
            if let Ok(user) = u_res {
                let new_student = db_models::users::NewStudent {
                    uid: user.uid,
                    school_id: student.school_id,
                    credit: student.credit,
                    accepted: student.accepted,
                    finished: student.finished,
                    major: &student.major,
                    year: student.year
                };
                use crate::schema::emtm_students;
                let res = diesel::insert_into(emtm_students::table)
                    .values(&new_student)
                    .execute(&self.connection);
                results.push(match res {
                    Ok(_) => {
                        let db_student = self.get_student_from_uid(user.uid);
                        match db_student {
                            Some(_student) => {
                                info!("Successfully added student with uid {}", user.uid);
                                Ok(user.uid)
                            }
                            None => {
                                warn!("Failed to add student with username {}, unknown error", student.username);
                                Err("unknown error".to_string())
                            }
                        }
                    }
                    Err(error) => {
                        info!("Failed to add student with uid {}: {}", user.uid, error);
                        Err(error.to_string())
                    }
                });
            } else if let Err(error) = u_res {
                results.push(Err(error.to_string()))
            }
        }

        results
    }


    fn add_db_user(&self, new_user: db_models::users::NewUser) -> Result<db_models::users::User, String> {
        use crate::schema::emtm_users;
        let result = diesel::insert_into(emtm_users::table)
            .values(&new_user)
            .execute(&self.connection);

        match result {
            Ok(_) => {
                let user = self.get_db_user_from_username(&new_user.username);
                match user {
                    Some(u) => {
                        info!("Successfully added user with uid {}", u.uid);
                        Ok(u)
                    }
                    None => {
                        warn!("Failed to add user with username {}, unknown error", new_user.username);
                        Err("unknown error".to_string())
                    }
                }
            }
            Err(error) => {
                info!("Failed to add user with username {}: {}", new_user.username, error);
                Err(error.to_string())
            }
        }
    }










    fn get_user_from_uid(&self, to_search: i32) -> Option<models::users::User> {
        use db_models::users::*;
        use crate::schema::emtm_users::dsl::*;

        let results = emtm_users
            .filter(uid.eq(to_search))
            .load::<User>(&self.connection);

        let db_u = {
            match results {
                Ok(mut users) => {
                    if users.is_empty() {
                        return None;
                    } else {
                        //Get first element
                        users.swap_remove(0)
                    }
                }
                Err(error) => {
                    error!("Panic when querying user with uid {}: {}", to_search, error);
                    panic!(error.to_string());
                }
            }
        };

        self.get_user_from_db_user(db_u)
    }


    fn get_user_from_username(&self, name: &str) -> Option<models::users::User> {
        use db_models::users::*;
        use crate::schema::emtm_users::dsl::*;

        let results = emtm_users
            .filter(username.eq(name))
            .load::<User>(&self.connection);

        let db_u = {
            match results {
                Ok(mut users) => {
                    if users.is_empty() {
                        return None;
                    } else {
                        //Get first element
                        users.swap_remove(0)
                    }
                }
                Err(error) => {
                    error!("Panic when querying user with username {}: {}", name, error);
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
                let db_c = self.get_cow_from_uid(db_u.uid);
                match db_c {
                    Some(c) => {
                        Some(User::Cow(models::users::Cow::from_db(db_u, c)))
                    }
                    None => {
                        warn!("Cow user with uid {} can't be found in cow table", db_u.uid);
                        None
                    }
                }
            }
            db_models::users::TYPE_STUDENT => {
                let db_s = self.get_student_from_uid(db_u.uid);
                match db_s {
                    Some(s) => {
                        Some(User::Student(models::users::Student::from_db(db_u, s)))
                    }
                    None => {
                        warn!("Cow user with uid {} can't be found in cow table", db_u.uid);
                        None
                    }
                }
            }
            _ => {
                warn!("Unexpected user type {} when querying uid {}", db_u.user_type, db_u.uid);
                None
            }
        }
    }

    fn get_db_user_from_username(&self, name: &str) -> Option<db_models::users::User> {
        use db_models::users::*;
        use crate::schema::emtm_users::dsl::*;

        let results = emtm_users
            .filter(username.eq(name))
            .load::<User>(&self.connection);
            match results {
                Ok(mut users) => {
                    if users.is_empty() {
                        None
                    } else {
                        //Get first element
                        Some(users.swap_remove(0))
                    }
                }
                Err(error) => {
                    error!("Panic when querying user with username {}: {}", name, error);
                    panic!(error.to_string());
                }
            }

    }


    fn get_cow_from_uid(&self, to_search: i32) -> Option<db_models::users::Cow> {
        use db_models::users::*;
        use crate::schema::emtm_cows::dsl::*;

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


    fn get_student_from_uid(&self, to_search: i32) -> Option<db_models::users::Student> {
        use db_models::users::*;
        use crate::schema::emtm_students::dsl::*;

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
                error!("Panic when querying student with uid {}: {}", to_search, error);
                panic!(error.to_string());
            }
        }
    }
}
