use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;

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
  /// filter option: account name
  pub account: Option<String>,

  #[arg(short, long)]
  /// filter option: purpose
  pub purpose: Option<Purpose>,

  #[arg(short('g'), long)]
  /// filter option: inclusive amount greater than (in cents)
  pub amount_greater_than: Option<u32>,

  #[arg(short('l'), long)]
  /// filter option: inclusive amount less than (in cents)
  pub amount_less_than: Option<u32>,

  #[arg(short, long)]
  /// filter option: only deleted operation
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
  #[command(visible_alias("a"))]
  Add {
    date: u32,
    note: String,
    amount: u32,
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
  #[command(visible_alias("c"))]
  Count,

  /// Delete (logically) operations
  #[command(visible_alias("d"))]
  Delete,

  /// List all operations
  #[command(visible_alias("l"))]
  List {
    #[arg(short, long, default_value = "pretty")]
    /// output format to display
    output_format: Option<Format>,
    #[arg(short, long)]
    /// include id field (only for pretty printing)
    include_id: bool,
    #[arg(short, long, default_value = "desc")]
    /// sort operations by date
    date_sort: Option<Sort>,
  },

  /// List all accounts
  #[command(visible_alias("i"))]
  ListAccount,

  /// Modify the filtered list
  #[command(visible_alias("m"))]
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
    amount: Option<u32>,
    #[arg(short, long)]
    /// new oepration date
    date: Option<u32>,
    #[arg(short, long)]
    /// new operation note
    note: Option<String>,
  },

  /// Undelete operations
  #[command(visible_alias("u"))]
  Undelete,

  /// Print current version
  #[command(visible_alias("v"))]
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
  None,
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

impl FromStr for Purpose {
  type Err = String;
  fn from_str(str_value: &str) -> Result<Self, String> {
    match str_value {
      "Need" => Ok(Purpose::Need),
      "Want" => Ok(Purpose::Want),
      "Goal" => Ok(Purpose::Goal),
      "Yearly Need" => Ok(Purpose::YearlyNeed),
      "Yearly Want" => Ok(Purpose::YearlyWant),
      "Income" => Ok(Purpose::Income),
      _ => Ok(Purpose::None),
    }
  }
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
      Purpose::None => String::new(),
    }
  }
}
