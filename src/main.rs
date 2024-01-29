mod cli;
mod commands;
mod data;

use clap::Parser;
use std::{env, fs};

use crate::cli::{Budgr, Command};
use crate::commands::*;

#[derive(Debug, Default)]
struct Config {
  pub data: String,
}

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

fn read_configuration(filename: &str) -> Config {
  let mut retvalue = Config::default();

  // try to read configuration file
  let config = fs::read_to_string(filename).unwrap();
  config
    .lines()
    .map(|line| {
      let mut parts = line.split('=');
      (parts.next().unwrap(), parts.next())
    })
    //handling single config keys
    .for_each(|kv_pair| match kv_pair.0 {
      "budgr.data" => {
        if let Some(value) = kv_pair.1 {
          retvalue.data = String::from(value);
        }
      }
      _ => unreachable!("All configurations should be covered for now."),
    });

  retvalue
}
