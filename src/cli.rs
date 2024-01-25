use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
pub struct Budgr {
  #[arg(short, long)]
  /// filter option: inclusive date from
  pub from: Option<u32>,
  #[arg(short, long)]
  /// filter option: inclusive date to
  pub to: Option<u32>,

  #[arg(short, long, default_value = "raw")]
  /// output format to display
  pub output_format: Option<Format>,

  #[command(subcommand)]
  pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// List all operations
  List,

  /// Count all operations
  Count,

  /// Print current version
  Version,

  /// Add a new operation
  Add {
    date: u32,
    note: String,
    amount: i32,
  },

  /// Modify the filtered list
  Modify {
    #[arg(short, long)]
    account: Option<String>,
    #[arg(short, long)]
    purpose: Option<String>,
    #[arg(short, long)]
    goal: Option<String>,
  },
}

#[derive(Subcommand, Debug)]
pub enum ModifyCommand {
  Account,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
  /// machine readable format, pipe ('|') separated
  Raw,
  Pretty,
}
