use crate::data::Data;
use crate::Budgr;
use crate::Command;
use crate::Config;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;

pub fn add_operation(config: &Config, args: &Budgr) {
  if let Some(Command::Add { date, note, amount }) = &args.command {
    let mut operation = Data {
      date: *date,
      note: String::from(note),
      amount: *amount,
      ..Data::default()
    };
    let mut rng = thread_rng();
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
