use crate::data::read_data;
use crate::Config;

pub fn list_operations(config: &Config) {
  let data = read_data(config);
  data.lines().for_each(|line| println!("{line}"));
}

pub fn count_operations(config: &Config) {
  let data = read_data(config);
  let count = data.lines().count() - 1;

  println!("{count}");
}
