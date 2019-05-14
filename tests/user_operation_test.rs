use emtm_db;
use emtm_db::controller::Controller;

#[test]
fn migration_test() {
    let ctrl = Controller::test_new();

    ctrl.revert_all();
    ctrl.migrate();
}
