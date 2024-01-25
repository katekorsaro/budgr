use crate::Config;
use crate::Budgr;
use crate::data::read_data;
use crate::commands::filter_data;

pub fn count_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  println!("{}", data.len());
}

