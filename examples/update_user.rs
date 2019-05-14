extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::controller::Controller;
use emtm_db::controller::school_controller_zh::SchoolControllerZh;
use emtm_db::controller::user_controller::UserController;

use emtm_db::models::users::*;

fn main() {
    pretty_env_logger::init_timed();

    let ctrl = Controller::new();

    let users = vec![
        User::Student(Student {
            uid: 13,
            wechat_id: "update0".to_string(),
            phone: "123".to_string(),
            personal_info: "12341234".to_string(),
            email: "aa@f.f".to_string(),
            username: "updat11".to_string(),
            verified: false,
            tokens: 0,
            school_id: ctrl.get_school_id("中山大学").unwrap(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "SE".to_string(),
            year: 0,
        }),
        User::Cow(Cow {
            uid: 1,
            wechat_id: "update1".to_string(),
            phone: "update1".to_string(),
            personal_info: "infoinfo".to_string(),
            email: "update1@uu.c".to_string(),
            username: "update1".to_string(),
            verified: false,
            tokens: 0,
            company: "sa".to_string(),
        }),
    ];

    println!("{:?}", ctrl.update_users(&users));
}
