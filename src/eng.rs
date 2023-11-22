use std::path::Path;

#[derive(PartialEq,Debug)]
enum EngErr {
    DataFileNotFound,
}

fn list (filename: &str) -> Result<Vec<String>, EngErr> {
    let path = Path::new(filename);

    if path.exists() == false {
        return Err(EngErr::DataFileNotFound);
    }

    Ok(Vec::<String>::new())
}

#[test]
fn list_call () {
    let _ = list("./testdata/db.csv");
}

#[test]
fn list_file_not_found () {
    let result = list("./testdata/non_existing_db.csv");
    assert_eq!(result, Err(EngErr::DataFileNotFound));
}
