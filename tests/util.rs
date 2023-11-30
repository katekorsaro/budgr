pub fn clean_test_env () {
    let dir = std::path::Path::new("./data/");
    let _ = std::fs::remove_dir_all(dir);
}

pub fn create_data_dir(){
    let dir = std::path::Path::new("./data/");

    if !dir.exists() {
        let _ = std::fs::create_dir(dir);
    }
}

pub fn copy_template_db_file () {
    let source_file = std::path::Path::new("./testdata/db.csv");
    let dest_file = std::path::Path::new("./data/db.csv");

    let _ = std::fs::copy(source_file, dest_file);
}

#[cfg(test)]
mod tests {
    #[test]
    fn util () {
        super::clean_test_env();
        super::create_data_dir();
        super::copy_template_db_file();
    }
}
