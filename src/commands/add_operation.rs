use crate::commands::write_operation::write_operation;
use crate::config::Config;
use crate::data::Operation;
use crate::Budgr;
use crate::Command;

use time::format_description;
use time::OffsetDateTime;

pub fn add_operation(config: &Config, args: &Budgr) {
  if let Some(Command::Add {
    date,
    note,
    amount,
    account,
    purpose,
    goal,
    id,
  }) = &args.command
  {
    let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let now = OffsetDateTime::now_local().unwrap().format(&fmt).unwrap();
    let operation = Operation {
      date: *date,
      note: String::from(note),
      amount: *amount,
      creation_date: now,
      account: account.clone(),
      purpose: purpose.as_ref().map(String::from),
      goal: goal.clone(),
      ..Operation::default()
    };
    write_operation(operation, *id, config);
  }
}
