/// version 0.0.1

/// GIVEN a CSV file with 3 fields
/// THEN I want to print read it line by line and print it on stdout

use std::fs;

fn print_file_content(c:&String){
    print!("{c}");
}

fn main() {
    let r = fs::read_to_string("test.csv");

    match r {
        Err(err) => println!("ERR: {err}"),
        Ok(result) => {
            print_file_content(&result);
        }
    }
}
