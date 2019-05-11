extern crate emtm_db;
extern crate env_logger;

use emtm_db::controller::Controller;


fn main() {
    env_logger::init();

    let ctrl = Controller::new();
    println!("{:?}", ctrl.get_user_from_uid(1));
}