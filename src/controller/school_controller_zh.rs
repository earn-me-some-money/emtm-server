use crate::controller::Controller;
use diesel::prelude::*;

#[derive(Queryable,Debug,Clone)]
pub struct School {
    pub school_id: i32,
    pub school_name: String
}

pub trait SchoolControllerZh {
    fn get_school_name(&self, id: i32) -> Option<String>;

    fn get_school_id(&self, name: &str) -> Option<i32>;

    fn get_school_list(&self) -> Vec<School>;
}

impl SchoolControllerZh for Controller {
    fn get_school_name(&self, id: i32) -> Option<String> {
        use crate::schema::school_zh::dsl::*;

        let results = school_zh
            .filter(school_id.eq(id))
            .load::<School>(&self.connection);

        match results {
            Ok(mut schools) => {
                if schools.is_empty() {
                    None
                } else {
                    Some(schools.swap_remove(0).school_name)
                }
            }
            Err(error) => {
                error!("Panic when querying school with id {} because: {}", id, error);
                panic!(error.to_string());
            }
        }
    }

    fn get_school_id(&self, name: &str) -> Option<i32> {
        use crate::schema::school_zh::dsl::*;

        let results = school_zh
            .filter(school_name.eq(name))
            .load::<School>(&self.connection);

        match results {
            Ok(mut schools) => {
                if schools.is_empty() {
                    None
                } else {
                    Some(schools.swap_remove(0).school_id)
                }
            }
            Err(error) => {
                error!("Panic when querying school with name {} because: {}", name, error);
                panic!(error.to_string());
            }
        }
    }

    fn get_school_list(&self) -> Vec<School> {
        use crate::schema::school_zh::dsl::*;

        let results = school_zh
            .load::<School>(&self.connection);

        match results {
            Ok(schools) => schools,
            Err(error) => {
                error!("Panic when getting school list");
                panic!(error.to_string());
            }
        }
    }
}
