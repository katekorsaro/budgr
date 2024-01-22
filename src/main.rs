use clap::{Parser, Subcommand};
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
struct Budgr {
  #[command(subcommand)]
  command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// List all operations
  List,

  /// Count all operations
  Count,
}

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
    Some(Command::List) => list_operations(&config),
    Some(Command::Count) => count_operations(&config),
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

fn list_operations(config: &Config) {
  let data = fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap();
  data.lines().for_each(|line| println!("{line}"));
}

fn count_operations(config: &Config) {
  let data = fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap();
  let count = data.lines().count() - 1;

  println!("{count}");
}
