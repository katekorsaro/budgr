use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
pub struct Budgr {
  #[arg(short, long)]
  /// filter option: inclusive date from
  pub from: Option<u32>,
  #[arg(short, long)]
  /// filter option: inclusive date to
  pub to: Option<u32>,
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
