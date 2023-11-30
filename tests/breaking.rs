#[test]
fn breaking_init () {
    budgr::init();
}

#[test]
fn breaking_list () {
    let _ops: Result<Vec<budgr::Op>, String> = budgr::list();
}
