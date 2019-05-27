extern crate emtm_db;
extern crate pretty_env_logger;
use emtm_db::controller::Controller;
use emtm_db::search;

fn main() {
    pretty_env_logger::try_init_timed_custom_env("EMTM_LOG").unwrap();

    let ctrl = Controller::new();
    use std::ops::Deref;
    println!(
        "{:?}",
        search::query_mission("advert")
    );
}
