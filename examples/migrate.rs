extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::Controller;

fn main() {
    pretty_env_logger::init_timed();

    let ctrl = Controller::new();

    ctrl.revert_all();
    ctrl.migrate();
}
