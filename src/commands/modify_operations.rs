use crate::commands::filter_data::filter_data;
use crate::data::read_data;
use crate::Budgr;
use crate::Command;
use crate::Config;
use std::fs::OpenOptions;
use std::io::Write;
use time::format_description;
use time::OffsetDateTime;

pub fn modify_operations(config: &Config, args: &Budgr) {
  if let Some(Command::Modify {
    account,
    purpose,
    goal,
    amount,
    date,
    note,
  }) = &args.command
  {
    let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let now = OffsetDateTime::now_local().unwrap().format(&fmt).unwrap();
    let data = read_data(config);
    let mut data = filter_data(data, args);
    let mut count = 0;
    data.iter_mut().for_each(|operation| {
      if let Some(account) = account {
        operation.account = Some(String::from(account));
      }
      if let Some(purpose) = purpose {
        operation.purpose = Some(String::from(purpose));
      }
      if let Some(goal) = goal {
        operation.goal = Some(String::from(goal));
      }
      operation.amount = amount.unwrap_or(operation.amount);
      operation.date = date.unwrap_or(operation.date);
      operation.note = note.clone().unwrap_or(operation.note.clone());
      operation.modification_date = Some(now.clone());
      // let's write to file!
      let string_value = operation.to_raw_string();
      let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&operation.path)
        .unwrap();

      let _ = file.write_all(string_value.as_bytes());
      count += 1;
    });
    println!("{count} operation(s) affected");
  }
}
