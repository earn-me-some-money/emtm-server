use emtm_db;
use emtm_db::controller::Controller;
use emtm_db::controller::UserController;
use emtm_db::models::users::*;

#[test]
fn migration_test() {
    let ctrl = Controller::test_new();

    ctrl.revert_all();
    ctrl.migrate();
}

#[test]
fn add_cow_test() {
    let ctrl = Controller::test_new();

    ctrl.revert_all();
    ctrl.migrate();

    let mut cows = vec![
        Cow {
            uid: 0,
            wechat_id: "cow1".to_string(),
            phone: "12312312312".to_string(),
            personal_info: "nono".to_string(),
            email: "cow1@test".to_string(),
            username: "cow1".to_string(),
            verified: false,
            tokens: 0,
            company: "sun".to_string(),
        },
        Cow {
            uid: 0,
            wechat_id: "cow2".to_string(),
            phone: "12312312312".to_string(),
            personal_info: "okok".to_string(),
            email: "cow2@test".to_string(),
            username: "cow2".to_string(),
            verified: false,
            tokens: 0,
            company: "san".to_string(),
        },
    ];

    let mut add_res = ctrl.add_cows(cows.clone());
    cows[0].uid = add_res.remove(0).unwrap();
    cows[1].uid = add_res.remove(0).unwrap();

    let cow1 = ctrl.get_user_from_uid(cows[0].uid).unwrap();
    let cow2 = ctrl.get_user_from_username(&cows[1].username).unwrap();

    assert_eq!(User::Cow(cows.remove(0)), cow1);
    assert_eq!(User::Cow(cows.remove(0)), cow2);
}
