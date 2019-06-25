use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;

use emtm_db;
use emtm_db::controller::mission_controller::MissionController;
use emtm_db::controller::survey_controller::SurveyController;
use emtm_db::controller::user_controller::UserController;
use emtm_db::controller::Controller;

use emtm_db::models::missions::*;
use emtm_db::models::survey::*;
use emtm_db::models::users::*;

#[test]
fn add_questions_test() {
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
        max_participants: Some(5),
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None,
        min_credit: None,
        major: None,
    };
    ctrl.add_mission(&mission).unwrap();

    let question = vec![Question {
        mid: 1,
        description: "test".to_string(),
        choices: QuestionContent::Blank,
    }];

    ctrl.add_questions(question, 1).unwrap();

    let res = ctrl.get_questionnaire(1).unwrap();
    assert_eq!(res[0].description, "test");
}

#[test]
fn add_answer_test() {
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
        max_participants: Some(5),
        min_grade: None,
        max_grade: None,
        school: None,
        min_finished: None,
        min_credit: None,
        major: None,
    };
    ctrl.add_mission(&mission).unwrap();

    let question = vec![Question {
        mid: 1,
        description: "test".to_string(),
        choices: QuestionContent::Blank,
    }];

    ctrl.add_questions(question, 1).unwrap();

    let mut students = vec![Student {
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
    }];

    let mut add_res = ctrl.add_students(&students);
    students[0].uid = add_res.remove(0).unwrap();
    let answer_content = vec![AnswerContent::Blank("test answer".to_string())];
    let answer = Answer {
        mid: 1,
        user_id: students[0].uid,
        user_answer: answer_content,
    };
    ctrl.add_answer(&answer).unwrap();

    let get_question_answers = ctrl.get_question_answers(1);
    let get_student_answered = ctrl.get_student_answered(students[0].uid);
    let get_student_answer = ctrl.get_student_answer(1, students[0].uid).unwrap();

    assert_eq!(
        get_question_answers[0].user_answer[0],
        AnswerContent::Blank("test answer".to_string())
    );
    assert_eq!(get_student_answered[0], 1);
    assert_eq!(
        get_student_answer.user_answer[0],
        AnswerContent::Blank("test answer".to_string())
    );
}
