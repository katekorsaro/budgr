pub fn init() {
    let dir = std::path::Path::new("./data/");
    let db_file = std::path::Path::new("./data/db.csv");

    if !dir.exists() {
        let res = std::fs::create_dir(dir);

        if res.is_err() {
            panic!("ERR: while creating ./data directory");
        }
    }

    if !db_file.exists() {
        let res = std::fs::File::create(db_file);

        if res.is_err() {
            panic!("ERR: while creating ./data/db.csv");
        }
    }
}
