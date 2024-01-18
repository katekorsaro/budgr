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
    let cmd = Budgr::parse();
}
