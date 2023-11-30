use std::path::Path;

mod util;

#[test]
fn integration_init () {
    util::clean_test_env();

    let dir = Path::new("./data/");
    let db_file = Path::new("./data/db.csv");

    assert!(!dir.exists());

    budgr::init();

    assert!(dir.exists());
    assert!(db_file.exists());

    util::clean_test_env();
}
