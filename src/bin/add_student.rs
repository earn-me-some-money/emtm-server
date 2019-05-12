extern crate emtm_db;
extern crate env_logger;

use emtm_db::controller::controller::Controller;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::school_controller_zh::SchoolControllerZh;

use emtm_db::models::users::*;

fn main() {
    env_logger::init();

    let ctrl = Controller::new();


    let students = vec![
        Student {
            uid: 0,
            wechat_id: "ello".to_string(),
            phone: "123".to_string(),
            personal_info: "12341234".to_string(),
            email: "aa@f.f".to_string(),
            username: "ads123".to_string(),
            verified: false,
            tokens: 0,
            school_id: ctrl.get_school_id("中山大学").unwrap(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "SE".to_string(),
            year: 0
        },
        Student {
            uid: 1,
            wechat_id: "adsgasd".to_string(),
            phone: "1242314".to_string(),
            personal_info: "lkafsd".to_string(),
            email: "bb@#lkasdfgj".to_string(),
            username: "ads1f123".to_string(),
            verified: false,
            tokens: 0,
            school_id: ctrl.get_school_id("华南理工大学").unwrap(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "CS".to_string(),
            year: 0
        }
    ];

    println!("{:?}", ctrl.add_students(students));
}