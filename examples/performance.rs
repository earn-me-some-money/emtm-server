extern crate emtm_db;
extern crate pretty_env_logger;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use emtm_db::controller::mission_controller::MissionController;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::Controller;
use emtm_db::models::missions::*;
use emtm_db::models::users::*;
use emtm_db::search;
use rand;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    pretty_env_logger::try_init_timed_custom_env("EMTM_LOG").unwrap();

    let ctrl = Controller::test_new();
    ctrl.revert_all();
    ctrl.migrate();

    let mut cows = vec![Cow {
        uid: 0,
        wechat_id: "cow1".to_string(),
        phone: "12312312312".to_string(),
        personal_info: "nono".to_string(),
        email: "cow1@test".to_string(),
        username: "cow1".to_string(),
        verified: false,
        tokens: 0,
        company: "sun".to_string(),
    }];

    let mut add_cow_res = ctrl.add_cows(&cows);
    cows[0].uid = add_cow_res.remove(0).unwrap();

    let participants = vec![];
    let d = NaiveDate::from_ymd(2015, 6, 3);
    let t1 = NaiveTime::from_hms_milli(12, 34, 56, 789);
    let t2 = NaiveTime::from_hms_milli(14, 34, 56, 789);

    let post_time = NaiveDateTime::new(d, t1);
    let deadline = NaiveDateTime::new(d, t2);
    let mut rng = rand::thread_rng();
    let start = SystemTime::now();
    for _i in 0..10000 {
        let mission = Mission {
            mid: 0,
            poster_uid: cows[0].uid,
            bounty: rng.gen_range(0, 100),
            risk: rng.gen_range(0, 200),
            name: (0..20)
                .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
                .collect(),
            mission_type: MissionType::Questionnaire,
            content: (0..500)
                .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
                .collect(),
            post_time,
            deadline,
            participants:participants.clone(),
            max_participants: 5,
            min_grade: None,
            max_grade: Some(rng.gen_range(0, 10)),
            school: None,
            min_finished: Some(rng.gen_range(0, 1000000))
        };
        ctrl.add_mission(&mission).unwrap();
    }
    search::commit_change().unwrap();
    println!("Insert Time: {}ms", start.elapsed().unwrap().as_millis());

    let mut mission_list = ctrl.get_missions_list();
    for mission in &mut mission_list {
        mission.content = mission.content.clone() + " Updated";
        ctrl.update_mission(&mission).unwrap();
    }
    search::commit_change().unwrap();
    println!("{:?}", search::query_mission("aa"));
}
