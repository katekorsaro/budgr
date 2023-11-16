/// version 0.0.1

/// GIVEN a CSV file with 3 fields
/// THEN I want to print read it line by line and print it on stdout
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("data/test.csv")?;
    let mut r = BufReader::new(f);

    loop {
        let mut line:String = String::new();
        let result = r.read_line(&mut line);

        match result {
            Ok(0) => break,
            _ => print!("{line}"),
        }
    }
    Ok(())
}
