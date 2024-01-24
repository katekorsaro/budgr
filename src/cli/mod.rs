use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
pub struct Budgr {
  #[command(subcommand)]
  pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// List all operations
  List,

  /// Count all operations
  Count,
}
