use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use crate::obj::Op;

#[derive(PartialEq,Debug)]
enum EngErr {
    DataFileNotFound,
    DataFileNotReadable,
    WhileReadingFile,
}

fn list (filename: &str) -> Result<Vec<Op>, EngErr> {
    let path = Path::new(filename);

    if path.exists() == false {
        return Err(EngErr::DataFileNotFound);
    }

    let file = File::open(filename);
    let file = match file {
        Err(_) => return Err(EngErr::DataFileNotReadable),
        Ok(f)=> f,
    };

    let mut ret = Vec::<Op>::new();

    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(_) => ret.push(Op::from("")),
            Err(_) => return Err(EngErr::WhileReadingFile),
        }
    }

    Ok(ret)
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

#[test]
fn list_row_number () {
    let result = list("./testdata/db.csv");
    match result {
        Ok(ops) => assert_eq!(ops.len(), 30),
        Err(_) => assert!(false),
    }
}
