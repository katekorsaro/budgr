use budgr::*;
use std::path::Path;
use std::fs;

pub fn show_help () {
    println!("budgr: a personal CLI budget manager");
}

pub fn handle_command(args:Vec<String>) {
    let cmd = match args[1].as_str() {
        // list
        "ls" => "ls",
        "list" => "ls",
        // init
        "init" => "init",
        // no operation
        _ => "nop",
    };

    match cmd {
        "ls" => list_operations(&args[2..]),
        "init" => init_db(),
        _ => println!("ERR: command not defined"),
    }
}

fn list_operations (_params:&[String]) {
    let ops = list("./data/db.csv");

    if ops.is_err() {
        panic!();
    }

    let ops = ops.unwrap();
    let ops = ops.iter();

    ops.for_each(|x| println!("{x:?}"));
}

fn init_db () {
    let path = Path::new("./data/");

    if !path.exists() {
        let _ = fs::create_dir(path);
    }

    let path = Path::new("./data/db.csv");

    if !path.exists() {
        let _ = fs::File::create("./data/db.csv");
    }
}
