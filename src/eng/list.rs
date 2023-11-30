use crate::eng::EngErr;
use crate::obj::Op;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn list(filename: &str) -> Result<Vec<Op>, EngErr> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(EngErr::DataFileNotFound);
    }

    let file = File::open(filename);
    let file = match file {
        Err(_) => return Err(EngErr::DataFileNotReadable),
        Ok(f) => f,
    };

    let mut ret = Vec::<Op>::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => ret.push(Op::from(&line)),
            Err(_) => return Err(EngErr::WhileReadingFile),
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_call() {
        let _ = list("./testdata/db.csv");
    }

    #[test]
    fn list_file_not_found() {
        let result = list("./testdata/non_existing_db.csv");
        assert_eq!(result, Err(EngErr::DataFileNotFound));
    }

    #[test]
    fn list_row_number() {
        let result = list("./testdata/db.csv");
        match result {
            Ok(ops) => assert_eq!(ops.len(), 30),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn list_first_row() {
        let result = list("./testdata/db.csv");
        match result {
            Err(_) => panic!(),
            Ok(ops) => {
                assert_eq!(ops[0].id, 1);
                assert_eq!(ops[0].date, 20230101);
                assert_eq!(ops[0].val, 1000);
                assert_eq!(ops[0].desc, String::from("Description for operation 001"));

                assert_eq!(ops[15].id, 16);
                assert_eq!(ops[15].date, 20230116);
                assert_eq!(ops[15].val, -1000);
                assert_eq!(ops[15].desc, String::from("Description for operation 001"));
            }
        }
    }
}
