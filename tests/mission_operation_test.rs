use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use emtm_db;
use emtm_db::controller::mission_controller::MissionController;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::Controller;
use emtm_db::models::missions::*;
use emtm_db::models::users::*;

#[test]
fn add_update_mission_test() {
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
        mission_type: MissionType::Questionnaire,
        content: "question".to_string(),
        post_time: post_time,
        deadline: deadline,
        participants: participants,
        max_participants: 5,
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None,
    };
    ctrl.add_mission(&mission).unwrap();
    let mission_list = ctrl.get_poster_missions(cows[0].uid);
    assert_eq!(mission_list[0].name, "test");
    assert_eq!(mission_list[0].content, "question");

    let participants = vec![];
    let new_mission = Mission {
        mid: 1,
        poster_uid: cows[0].uid,
        bounty: 0,
        risk: 0,
        name: "update".to_string(),
        mission_type: MissionType::Questionnaire,
        content: "question updated".to_string(),
        post_time: post_time,
        deadline: deadline,
        participants: participants,
        max_participants: 5,
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None,
    };
    ctrl.update_mission(&new_mission).unwrap();
    let mission_list = ctrl.get_poster_missions(cows[0].uid);

    assert_eq!(mission_list[0].name, "update");
    assert_eq!(mission_list[0].content, "question updated");
}

#[test]
fn add_update_participants_test() {
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

    let mut students = vec![
        Student {
            uid: 0,
            wechat_id: "student1".to_string(),
            phone: "12312312302".to_string(),
            personal_info: "ok".to_string(),
            email: "stduent1@test".to_string(),
            username: "student1".to_string(),
            verified: false,
            tokens: 0,
            school_id: 11311,
            student_id: "16340000".to_string(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "testing".to_string(),
            year: 1,
        },
        Student {
            uid: 0,
            wechat_id: "student2".to_string(),
            phone: "12312312213".to_string(),
            personal_info: "no".to_string(),
            email: "stduent2@test".to_string(),
            username: "student2".to_string(),
            verified: false,
            tokens: 0,
            school_id: 11311,
            student_id: "16340001".to_string(),
            credit: 0,
            accepted: 0,
            finished: 0,
            major: "testing".to_string(),
            year: 1,
        },
    ];
    let mut add_res = ctrl.add_students(&students);
    students[0].uid = add_res.remove(0).unwrap();
    students[1].uid = add_res.remove(0).unwrap();

    let d = NaiveDate::from_ymd(2015, 6, 3);
    let t1 = NaiveTime::from_hms_milli(12, 34, 56, 789);
    let t2 = NaiveTime::from_hms_milli(14, 34, 56, 789);
    let post_time = NaiveDateTime::new(d, t1);
    let deadline = NaiveDateTime::new(d, t2);

    let participants = vec![];
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
        min_finished: None,
    };
    ctrl.add_mission(&mission).unwrap();

    let add_participants = vec![
        Participant {
            student_uid: students[0].uid,
            state: PartState::Accepted,
        },
        Participant {
            student_uid: students[1].uid,
            state: PartState::Accepted,
        },
    ];
    ctrl.add_participants(1, &add_participants).unwrap();

    let par_list = ctrl.get_mission_participants(1);
    assert_eq!(par_list[0].student_uid, students[0].uid);
    assert_eq!(par_list[1].student_uid, students[1].uid);

    let update_par_0 = Participant {
        student_uid: students[0].uid,
        state: PartState::Finished,
    };
    let update_par_1 = Participant {
        student_uid: students[1].uid,
        state: PartState::Cancelled,
    };
    ctrl.update_participant(1, &update_par_0).unwrap();
    ctrl.update_participant(1, &update_par_1).unwrap();

    let par_list = ctrl.get_mission_participants(1);
    assert_eq!(par_list[0].state, 1);
    assert_eq!(par_list[1].state, 2);
}
