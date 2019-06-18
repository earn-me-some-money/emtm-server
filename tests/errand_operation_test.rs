use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use emtm_db;
use emtm_db::controller::Controller;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::mission_controller::MissionController;
use emtm_db::controller::errand_controller::ErrandController;
use emtm_db::models::errand::*;
use emtm_db::models::users::*;
use emtm_db::models::missions::*;

#[test]
fn add_errand_test(){
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
    let mission = Mission {
        mid: 0,
        poster_uid: cows[0].uid,
        bounty: 0,
        risk: 0,
        name: "test".to_string(),
        mission_type: MissionType::Errands,
        content: "question".to_string(),
        post_time: post_time,
        deadline: deadline,
        participants: participants,
        max_participants: Some(5),
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None,
        min_credit: None,
        major: None,
    };
    ctrl.add_mission(&mission).unwrap();

    let errand = Errand{
        mid: 1,
        pickup_address: "s".to_string(),
        phone_number: "12334454665".to_string(),
        item_code: Some("133".to_string()),
        deliver_address: "a".to_string(),
        other_info: "no".to_string(),
    };
    ctrl.add_errand(&errand).unwrap();

    let res = ctrl.get_errand(1).unwrap();
    
    assert!(true);
    assert_eq!(res.pickup_address, "s");
}
