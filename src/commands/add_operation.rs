use crate::config::Config;
use crate::data::Operation;
use crate::Budgr;
use crate::Command;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;
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
    let mut rng = thread_rng();
    let mut operation = Operation {
      date: *date,
      note: String::from(note),
      amount: *amount,
      creation_date: now,
      account: account.clone(),
      purpose: purpose.as_ref().map(String::from),
      goal: goal.clone(),
      ..Operation::default()
    };
    let mut force = false;
    loop {
      let id = if let Some(id) = id {
        force = true;
        *id
      } else {
        rng.gen::<u32>()
      };
      let filename = format!("{}{}.{:010}.bgr", config.data, date, id);
      let file = OpenOptions::new()
        .create(force) // in case of fixed id
        .create_new(!force) // in case of rnd id
        .write(true)
        .truncate(true)
        .open(&filename);
      if let Ok(mut file) = file {
        operation.id = id;
        let string_value = operation.to_raw_string();
        let _ = file.write_all(string_value.as_bytes());
        break;
      }
    }
  }
}
