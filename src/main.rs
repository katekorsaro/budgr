use std::env;
use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command()]
struct Budgr {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all operations
    List,
}

fn main () {
    // read env variable to get configuration
    let budgrrc = env::vars()
        .find(|(k, _)| k == "BUDGRRC" );

    // store env variable into an actual variable
    let budgrrc = match budgrrc {
        Some((_, value)) => value,
        None => String::from(".//.budgrrc//"),
    };

    println!("{budgrrc}");

    let _cmd = Budgr::parse();
}
