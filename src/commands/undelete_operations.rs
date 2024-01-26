use crate::commands::filter_data::filter_data;
use crate::data::read_data;
use crate::data::Status;
use crate::fs::OpenOptions;
use crate::Budgr;
use crate::Config;
use std::io::Write;
use time::{format_description, OffsetDateTime};

pub fn undelete_operations(config: &Config, args: &Budgr) {
  let fmt = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
  let now = OffsetDateTime::now_local().unwrap().format(&fmt).unwrap();
  let data = read_data(config);
  let mut data = filter_data(data, args);
  data.iter_mut().for_each(|operation| {
    operation.status = Status::Active;
    operation.modification_date = Some(now.clone());
    let string_value = operation.to_raw_string();
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(&operation.path)
      .unwrap();
    let _ = file.write_all(string_value.as_bytes());
  });
}
