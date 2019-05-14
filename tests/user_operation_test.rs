use emtm_db;
use emtm_db::controller::Controller;
use emtm_db::controller::UserController;
use emtm_db::models::users::*;


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
            phone: "123123123154".to_string(),
            personal_info: "okok".to_string(),
            email: "cow2@test".to_string(),
            username: "cow2".to_string(),
            verified: false,
            tokens: 0,
            company: "san".to_string(),
        },
    ];

    let mut add_res = ctrl.add_cows(&cows);
    cows[0].uid = add_res.remove(0).unwrap();
    cows[1].uid = add_res.remove(0).unwrap();

    use emtm_db::models::users::UserId;
    let cow1 = ctrl.get_user_from_identifier(UserId::Uid(cows[0].uid)).unwrap();
    let cow2 = ctrl.get_user_from_identifier(UserId::WechatId(&cows[1].wechat_id)).unwrap();

    assert_eq!(User::Cow(cows.remove(0)), cow1);
    assert_eq!(User::Cow(cows.remove(0)), cow2);
}
