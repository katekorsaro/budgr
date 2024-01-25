use crate::commands::filter_data;
use crate::data::read_data;
use crate::Budgr;
use crate::Command;
use crate::Config;
use std::fs::OpenOptions;
use std::io::Write;

pub fn modify_operations(config: &Config, args: &Budgr) {
  if let Some(Command::Modify {
    account,
    purpose,
    goal,
  }) = &args.command
  {
    let data = read_data(config);
    let mut data = filter_data(data, args);
    data.iter_mut().for_each(|operation| {
      if let Some(account) = account {
        operation.account = Some(String::from(account))
      };
      if let Some(purpose) = purpose {
        operation.purpose = Some(String::from(purpose))
      };
      if let Some(goal) = goal {
        operation.goal = Some(String::from(goal))
      };
      // let's write to file!
      let string_value = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        operation.id,
        operation.date,
        operation.note,
        operation.amount,
        operation.account.clone().unwrap_or("".to_string()),
        operation.purpose.clone().unwrap_or("".to_string()),
        operation.goal.clone().unwrap_or("".to_string())
      );
      let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&operation.path)
        .unwrap();

      let _ = file.write_all(string_value.as_bytes());
    });
  }
}
