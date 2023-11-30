use std::path::Path;
use std::fs;

pub fn clean_test_env () {
    let dir = Path::new("./data/");
    let _ = fs::remove_dir_all(dir);
}
