extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::Controller;
use emtm_db::controller::SchoolControllerZh;
use emtm_db::controller::UserController;

use emtm_db::models::users::UserId::*;

fn main() {
    pretty_env_logger::init_timed();

    let ctrl = Controller::new();
    println!("{:?}", ctrl.get_user_from_identifier(Uid(1)));
    println!("{:?}", ctrl.get_user_from_identifier(Uid(7)));
    match ctrl.get_db_student_from_uid(2) {
        Some(u) => {
            println!("{:?}", ctrl.get_school_name(u.school_id));
        }
        _ => (),
    }
    println!("{:?}", ctrl.get_user_from_identifier(Uid(12)));
    println!("{:?}", ctrl.get_user_from_identifier(WechatId("adsf")));

    println!("{:?}", ctrl.get_school_name(10558));
    println!("{:?}", ctrl.get_school_list().len());
}
