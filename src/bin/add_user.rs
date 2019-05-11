extern crate emtm_db;
extern crate env_logger;

use emtm_db::controller::*;
use emtm_db::controller::user_controller::UserController;


fn main() {
    env_logger::init();

    let ctrl = Controller::new();
    println!("{:?}", ctrl.add_user("123", "adfsfasd", " adfsf12", "14rfd", false, 0));
}