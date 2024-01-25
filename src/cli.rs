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

  #[command(subcommand)]
  pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// List all operations
  List {
    #[arg(short, long, default_value = "raw")]
    /// output format to display
    output_format: Option<Format>,
  },

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
    purpose: Option<Purpose>,
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

#[derive(Clone, Debug, ValueEnum)]
pub enum Purpose {
  Need,
  Want,
  Goal,
  YearlyNeed,
  YearlyWant,
}

impl From<&Purpose> for String {
  fn from(p: &Purpose) -> Self {
    match p {
      Purpose::Need => String::from("Need"),
      Purpose::Want => String::from("Want"),
      Purpose::Goal => String::from("Goal"),
      Purpose::YearlyNeed => String::from("Yearly Need"),
      Purpose::YearlyWant => String::from("Yearly Want"),
    }
  }
}
