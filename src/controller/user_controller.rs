use crate::controller::Controller;
use crate::db_models;
use crate::models;
use diesel::prelude::*;

pub trait UserController {
    fn add_cows(&self, cows: Vec<models::users::Cow>) -> Vec<Result<(), String>>;
    fn add_user(&self, wechat_id: &str, phone: &str, personal_info: &str,
                username: &str, verified: bool, tokens: i32) -> Result<db_models::users::User, String>;
    fn get_user_from_uid(&self, to_search: i32) -> Option<db_models::users::User>;
    fn get_user_from_username(&self, name: &str) -> Option<db_models::users::User>;
    fn get_cow_from_uid(&self, to_search: i32) -> Option<db_models::users::Cow>;
    fn add_db_user(&self, new_user: db_models::users::NewUser) -> Result<db_models::users::User, String>;
}

impl UserController for Controller {
    fn add_cows(&self, cows: Vec<models::users::Cow>) -> Vec<Result<(), String>> {
        let mut u_results = vec![];

        for cow in &cows {
            info!("Adding cow with username {}", cow.username);
            let db_u = db_models::users::NewUser {
                wechat_id: &cow.wechat_id,
                phone: &cow.phone,
                personal_info: &cow.personal_info,
                username: &cow.username,
                verified: cow.verified,
                tokens: cow.tokens,
            };
            u_results.push(self.add_db_user(db_u))
        }

        let mut results = vec![];
        for (cow, u_res) in cows.iter().zip(u_results.iter()) {
            match u_res {
                Ok(user) => {
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
                                    Ok(())
                                }
                                None => {
                                    warn!("Failed to add cow with username {}, unknown error", cow.username);
                                    Err("unknown error".to_string())
                                }
                            }
                        }
                        Err(error) => {
                            info!("Failed to add cow with uid {} because: {}", user.uid, error);
                            Err(error.to_string())
                        }
                    });
                }
                Err(error) => {
                    results.push(Err(error.to_string()))
                }
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
                let user = self.get_user_from_username(&new_user.username);
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
                info!("Failed to add user with username {} because: {}", new_user.username, error);
                Err(error.to_string())
            }
        }
    }


    fn add_user(&self, wechat_id: &str, phone: &str, personal_info: &str,
                username: &str, verified: bool, tokens: i32) -> Result<db_models::users::User, String> {
        info!("adding user with username {}", username);
        let new_user = db_models::users::NewUser {
            wechat_id,
            phone,
            personal_info,
            username,
            verified,
            tokens,
        };
        use crate::schema::emtm_users;
        let result = diesel::insert_into(emtm_users::table)
            .values(&new_user)
            .execute(&self.connection);

        match result {
            Ok(s) => {
                println!("{}", s);
                let user = self.get_user_from_username(username);
                match user {
                    Some(u) => {
                        info!("Successfully added user with uid {}", u.uid);
                        Ok(u)
                    }
                    None => {
                        warn!("Failed to add user with username {}, unknown error", username);
                        Err("unknown error".to_string())
                    }
                }
            }
            Err(error) => {
                info!("Failed to add user with username {} because: {}", username, error);
                Err(error.to_string())
            }
        }
    }


    fn get_user_from_uid(&self, to_search: i32) -> Option<db_models::users::User> {
        use db_models::users::*;
        use crate::schema::emtm_users::dsl::*;

        let results = emtm_users
            .filter(uid.eq(to_search))
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
                error!("Panic when querying user with uid {} because: {}", to_search, error);
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
                error!("Panic when querying cow with uid {} because: {}", to_search, error);
                panic!(error.to_string());
            }
        }
    }


    fn get_user_from_username(&self, name: &str) -> Option<db_models::users::User> {
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
                error!("Panic when querying user with username {} because: {}", name, error);
                panic!(error.to_string());
            }
        }
    }
}
