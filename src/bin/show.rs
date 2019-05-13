extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::Controller;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::school_controller_zh::SchoolControllerZh;

fn main() {
    pretty_env_logger::init_timed();

    let ctrl = Controller::new();
    println!("{:?}", ctrl.get_user_from_uid(1));
    println!("{:?}", ctrl.get_user_from_uid(7));
    match ctrl.get_student_from_uid(2) {
        Some(u) => {
            println!("{:?}", ctrl.get_school_name(u.school_id));
        }
        _ => ()
    }
    println!("{:?}", ctrl.get_user_from_uid(12));
    println!("{:?}", ctrl.get_user_from_username("adsf"));
    println!("{:?}", ctrl.get_user_from_username("asdfads"));

    println!("{:?}", ctrl.get_school_name(10558));
    println!("{:?}", ctrl.get_school_list().len());
}