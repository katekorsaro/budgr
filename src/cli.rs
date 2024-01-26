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

  #[arg(short, long)]
  /// only deleted operation
  pub deleted: bool,

  #[arg(short, long)]
  /// filter option: id
  pub id: Option<u32>,

  #[command(subcommand)]
  pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Add a new operation
  Add {
    date: u32,
    note: String,
    amount: i32,
    #[arg(short, long)]
    /// operation account
    account: Option<String>,
    #[arg(short, long)]
    /// operation purpose
    purpose: Option<Purpose>,
    #[arg(short, long)]
    /// operation goal
    goal: Option<String>,
    #[arg(short, long)]
    /// operation id (if the same id is already present for the same date, file will be overwritten)
    id: Option<u32>,
  },

  /// Count all operations
  Count,

  /// Delete (logically) operations
  Delete,

  /// List all operations
  List {
    #[arg(short, long, default_value = "raw")]
    /// output format to display
    output_format: Option<Format>,
    #[arg(short, long, default_value = "desc")]
    /// sort operations by date
    date_sort: Option<Sort>,
  },

  /// Modify the filtered list
  Modify {
    #[arg(short, long)]
    /// new operation account
    account: Option<String>,
    #[arg(short, long)]
    /// new operation purpose
    purpose: Option<Purpose>,
    #[arg(short, long)]
    /// new operation goal
    goal: Option<String>,
    #[arg(short('m'), long)]
    /// new operation purpose
    amount: Option<i32>,
    #[arg(short, long)]
    /// new oepration date
    date: Option<u32>,
    #[arg(short, long)]
    /// new operation note
    note: Option<String>,
  },

  /// Undelete operations
  Undelete,

  /// Print current version
  Version,
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
  Income,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Sort {
  Asc,
  Desc,
}

impl From<&Purpose> for String {
  fn from(p: &Purpose) -> Self {
    match p {
      Purpose::Need => String::from("Need"),
      Purpose::Want => String::from("Want"),
      Purpose::Goal => String::from("Goal"),
      Purpose::YearlyNeed => String::from("Yearly Need"),
      Purpose::YearlyWant => String::from("Yearly Want"),
      Purpose::Income => String::from("Income"),
    }
  }
}
