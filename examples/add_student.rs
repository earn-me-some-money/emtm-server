extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::school_controller_zh::SchoolControllerZh;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::Controller;

use emtm_db::models::users::*;

fn main() {
    pretty_env_logger::init_timed();

    let ctrl = Controller::new();

    let students = vec![
        Student {
            uid: 0,
            wechat_id: "ellaso".to_string(),
            phone: "123".to_string(),
            personal_info: "12341234".to_string(),
            email: "aa@f.f".to_string(),
            username: "ads123".to_string(),
            verified: false,
            tokens: 0,
            school_id: ctrl.get_school_id("中山大学").unwrap(),
            student_id: "16340023".to_string(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "SE".to_string(),
            year: 0,
        },
        Student {
            uid: 1,
            wechat_id: "adsg11asd".to_string(),
            phone: "1242314".to_string(),
            personal_info: "lkafsd".to_string(),
            email: "bb@#lkasdfgj".to_string(),
            username: "ads1f1柠檬🍋23".to_string(),
            verified: false,
            tokens: 0,
            school_id: ctrl.get_school_id("中山大学").unwrap(),
            student_id: "16340023".to_string(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "CS".to_string(),
            year: 0,
        },
    ];

    println!("{:?}", ctrl.add_students(students));
}
