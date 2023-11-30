use budgr::list;
use std::env;

mod cli;

fn main() {
    let args:Vec<String> = env::args().collect();

    match args.len() {
        0 => show_help(),
        _ => handle_command(args),
    }
}

fn show_help () {
    todo!();
}

fn handle_command(args:Vec<String>) {
    let cmd = match args[1].as_str() {
        "ls" => "ls",
        "list" => "ls",
        _ => "nop",
    };

    println!("{cmd}");

    todo!();
}
