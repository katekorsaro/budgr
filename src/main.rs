mod cli;
mod commands;
mod config;
mod data;

use clap::Parser;
use std::path::*;

use crate::cli::{Budgr, Command};
use crate::commands::*;
use crate::config::*;

fn main() {
    // read config
    let config = read_configuration();
    // create data dir
    let path = Path::new(&config.data);
    (!path.exists()).then(|| std::fs::create_dir(path));

    let bin = Budgr::parse();

    // handle commands
    match bin.command {
        Some(Command::Add { .. }) => add_operation(&config, &bin),
        Some(Command::Aggregate { .. }) => aggregate(&config, &bin),
        Some(Command::Count) => count_operations(&config, &bin),
        Some(Command::Delete) => delete_operations(&config, &bin),
        Some(Command::Import { .. }) => import_file(&config, &bin),
        Some(Command::List { .. }) => list_operations(&config, &bin),
        Some(Command::ListAccount) => list_accounts(&config, &bin),
        Some(Command::Modify { .. }) => modify_operations(&config, &bin),
        Some(Command::Undelete) => undelete_operations(&config, &bin),
        Some(Command::Version) => print_version(),
        _ => unreachable!("No other commands for now."),
    }
}
