use crate::data::Operation;
use crate::Budgr;
use crate::Command;
use crate::Config;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;
use time::format_description;
use time::OffsetDateTime;

pub fn add_operation(config: &Config, args: &Budgr) {
  if let Some(Command::Add { date, note, amount }) = &args.command {
    let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let now = OffsetDateTime::now_local().unwrap().format(&fmt).unwrap();
    let mut rng = thread_rng();
    let mut operation = Operation {
      date: *date,
      note: String::from(note),
      amount: *amount,
      creation_date: now,
      ..Operation::default()
    };
    loop {
      let id: u32 = rng.gen();
      let filename = format!("{}{}.{:010}.bgr", config.data, date, id);
      let file = OpenOptions::new()
        .create_new(true)
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
