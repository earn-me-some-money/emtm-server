use emtm_db;
use emtm_db::controller::Controller;
use emtm_db::controller::SchoolControllerZh;

#[test]
fn get_school_name_test() {
    let ctrl = Controller::test_new();

    ctrl.revert_all();
    ctrl.migrate();
    let res1 = ctrl.get_school_name(10001);
    let res2 = ctrl.get_school_name(10003);
    assert_eq!(res1.unwrap(), "北京大学");
    assert_eq!(res2.unwrap(), "清华大学");
}

#[test]
fn get_school_id_test() {
    let ctrl = Controller::test_new();

    ctrl.revert_all();
    ctrl.migrate();
    let res1 = ctrl.get_school_id("北京大学");
    let res2 = ctrl.get_school_id("清华大学");
    assert_eq!(res1.unwrap(), 10001);
    assert_eq!(res2.unwrap(), 10003);
}