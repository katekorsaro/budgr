pub fn list () -> Result<Vec<super::op::Op>, String> {
    let db = std::fs::read_to_string("./data/db.csv");

    if db.is_err() {
        return Err(String::from("ERR: couldn't read ./data/db.csv file"));
    }

    let db = db.unwrap();

    let mut res = Vec::<super::op::Op>::new();

    let ops_str:Vec<&str> = db.split('\n').collect();

    ops_str.iter()
        .filter(|x| !x.is_empty())
        .for_each(|x| res.push(super::op::Op::from(x)));

    Ok(res)
}
