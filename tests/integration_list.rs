mod util;

#[test]
fn integrtion_list () {
    util::clean_test_env();
    util::create_data_dir();
    util::copy_template_db_file();

    let ops = budgr::list();

    if ops.is_err() {
        panic!();
    }

    let ops = ops.unwrap();

    assert_eq!(ops.len(), 11);

    assert_eq!(ops[0].id, 1);
    assert_eq!(ops[0].date, 20230101);
    assert_eq!(ops[0].desc, String::from("Description 001"));
    assert_eq!(ops[0].val, -5);

    util::clean_test_env();
}
