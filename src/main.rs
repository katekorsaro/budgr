mod cli;
mod commands;
mod config;
mod data;

use clap::Parser;
use std::env;

use crate::cli::{Budgr, Command};
use crate::commands::*;
use crate::config::*;

fn main() {
  // read env variable to get configuration
  let budgrrc = env::vars().find(|(k, _)| k == "BUDGRRC");

  // store env variable into an actual variable
  let budgrrc = match budgrrc {
    Some((_, value)) => value,
    None => String::from(".//.budgrrc"),
  };

  let config = read_configuration(&budgrrc);

  let bin = Budgr::parse();

  // handle commands
  match bin.command {
    Some(Command::Add { .. }) => add_operation(&config, &bin),
    Some(Command::Count) => count_operations(&config, &bin),
    Some(Command::Delete) => delete_operations(&config, &bin),
    Some(Command::List { .. }) => list_operations(&config, &bin),
    Some(Command::ListAccount) => list_accounts(&config, &bin),
    Some(Command::Modify { .. }) => modify_operations(&config, &bin),
    Some(Command::Undelete) => undelete_operations(&config, &bin),
    Some(Command::Version) => print_version(),
    _ => unreachable!("No other commands for now."),
  }
}
