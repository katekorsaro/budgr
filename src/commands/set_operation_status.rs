use crate::commands::filter_data::filter_data;
use crate::config::Config;
use crate::data::{read_data, Status};
use crate::Budgr;
use std::fs::OpenOptions;
use std::io::Write;
use time::{format_description, OffsetDateTime};

pub fn set_operation_status(config: &Config, args: &Budgr, status: Status) {
  let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
  let now = OffsetDateTime::now_local().unwrap().format(&fmt).unwrap();
  let data = read_data(config);
  let mut data = filter_data(data, args);
  let mut count = 0;
  data.iter_mut().for_each(|operation| {
    operation.status = status;
    operation.modification_date = Some(now.clone());
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
