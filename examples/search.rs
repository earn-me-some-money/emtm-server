use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
extern crate emtm_db;
extern crate pretty_env_logger;
use emtm_db::controller::Controller;
use emtm_db::search;
use emtm_db::models::users::*;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::mission_controller::MissionController;
use emtm_db::models::missions::*;

fn main() {
    pretty_env_logger::try_init_timed_custom_env("EMTM_LOG").unwrap();

    let _ctrl = Controller::test_new();
    _ctrl.revert_all();
    _ctrl.migrate();
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

    let mut add_cow_res = _ctrl.add_cows(&cows);
    cows[0].uid = add_cow_res.remove(0).unwrap();

    let participants = vec![];
    let d = NaiveDate::from_ymd(2015, 6, 3);
    let t1 = NaiveTime::from_hms_milli(12, 34, 56, 789);
    let t2 = NaiveTime::from_hms_milli(14, 34, 56, 789);

    let post_time = NaiveDateTime::new(d, t1);
    let deadline = NaiveDateTime::new(d, t2);
    let mission = Mission {
        mid: 0,
        poster_uid: cows[0].uid,
        bounty: 0,
        risk: 0,
        name: "test".to_string(),
        mission_type: MissionType::Questionnaire,
        content: "question".to_string(),
        post_time: post_time,
        deadline: deadline,
        participants: participants,
        max_participants: 5,
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None
    };
    let participants = vec![];
    let mission2 = Mission {
        mid: 0,
        poster_uid: cows[0].uid,
        bounty: 0,
        risk: 0,
        name: "testing".to_string(),
        mission_type: MissionType::Questionnaire,
        content: "query".to_string(),
        post_time: post_time,
        deadline: deadline,
        participants: participants,
        max_participants: 5,
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None
    };
    _ctrl.add_mission(&mission).unwrap();
    _ctrl.add_mission(&mission2).unwrap();
    search::commit_change().unwrap();
    
    let res = search::query_mission("test").unwrap();

    println!("{:?}", res);
}
