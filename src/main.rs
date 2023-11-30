use std::env;
use cli::*;

mod cli;

fn main() {
    let args:Vec<String> = env::args().collect();

    match args.len() {
        1 => show_help(),
        _ => handle_command(args),
    }
}

