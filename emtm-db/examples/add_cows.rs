extern crate emtm_db;
extern crate pretty_env_logger;

use emtm_db::controller::Controller;
use emtm_db::controller::UserController;

use emtm_db::models::users::*;

fn main() {
    pretty_env_logger::try_init_timed_custom_env("EMTM_LOG").unwrap();

    let ctrl = Controller::new();

    let cows = vec![
        Cow {
            uid: 0,
            wechat_id: "ello".to_string(),
            phone: "123".to_string(),
            personal_info: "12341234".to_string(),
            email: "cajkhsd@jkhvdsz".to_string(),
            username: "adsf333".to_string(),
            verified: false,
            tokens: 0,
            company: "sun".to_string(),
        },
        Cow {
            uid: 1,
            wechat_id: "adsgasd".to_string(),
            phone: "1242314".to_string(),
            personal_info: "lkafsd".to_string(),
            email: "cajkhsd@jksz".to_string(),
            username: "ads1233f".to_string(),
            verified: false,
            tokens: 0,
            company: "sa".to_string(),
        },
    ];

    println!("{:?}", ctrl.add_cows(&cows));
}
