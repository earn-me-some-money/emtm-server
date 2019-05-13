use crate::db_models;

#[derive(Debug)]
pub struct Student {
    pub uid: i32,
    pub wechat_id: String,
    pub phone: String,
    pub personal_info: String,
    pub email: String,
    pub username: String,
    pub verified: bool,
    pub tokens: i32,
    pub school_id: i32,
    pub credit: i32,
    pub accepted: i32,
    pub finished: i32,
    pub major: String,
    pub year: i32,
}


impl Student {
    /// Create a User from a database user and database student
    pub fn from_db(u: db_models::users::User, s: db_models::users::Student) -> Self {
        Self {
            uid: u.uid,
            wechat_id: u.wechat_id,
            phone: u.phone,
            personal_info: u.personal_info,
            email: u.email,
            username: u.username,
            verified: u.verified,
            tokens: u.tokens,
            school_id: s.school_id,
            credit: s.credit,
            accepted: s.accepted,
            finished: s.finished,
            major: s.major,
            year: s.year,
        }
    }

    /// Get the user and student for database storage
    pub fn to_db(&self) -> (db_models::users::User, db_models::users::Student) {
        (db_models::users::User {
            uid: self.uid,
            wechat_id: self.wechat_id.clone(),
            phone: self.phone.clone(),
            personal_info: self.personal_info.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            verified: self.verified,
            tokens: self.tokens,
            user_type: 1
        }, db_models::users::Student {
            uid: self.uid,
            school_id: self.school_id,
            credit: self.credit,
            accepted: self.accepted,
            finished: self.finished,
            major: self.major.clone(),
            year: self.year,
        })
    }
}

#[derive(Debug)]
pub struct Cow {
    pub uid: i32,
    pub wechat_id: String,
    pub phone: String,
    pub personal_info: String,
    pub email: String,
    pub username: String,
    pub verified: bool,
    pub tokens: i32,
    pub company: String,
}

impl Cow {
    pub fn from_db(u: db_models::users::User, c: db_models::users::Cow) -> Self {
        Self {
            uid: u.uid,
            wechat_id: u.wechat_id,
            phone: u.phone,
            personal_info: u.personal_info,
            email: u.email,
            username: u.username,
            verified: u.verified,
            tokens: u.tokens,
            company: c.company
        }
    }
    pub fn to_db(&self) -> (db_models::users::User, db_models::users::Cow) {
        (db_models::users::User {
            uid: self.uid,
            wechat_id: self.wechat_id.clone(),
            phone: self.phone.clone(),
            personal_info: self.personal_info.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            verified: self.verified,
            tokens: self.tokens,
            user_type: 0
        }, db_models::users::Cow {
            uid: self.uid,
            company: self.company.clone()
        })
    }
}

#[derive(Debug)]
pub enum User {
    Student(Student),
    Cow(Cow)
}

