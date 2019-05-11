extern crate emtm_db;
extern crate env_logger;

use emtm_db::controller::Controller;

use emtm_db::models::users::*;

fn main() {
    env_logger::init();

    let ctrl = Controller::new();

    let cows = vec![

    ]

    println!("{:?}", ctrl.add_cows("123", "adfsfasd", " adfsf12", "14rfd", false, 0));
}