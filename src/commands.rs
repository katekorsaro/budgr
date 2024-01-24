use crate::data::read_data;
use crate::Config;

pub fn list_operations(config: &Config) {
  let data = read_data(config);
  data.into_iter().for_each(|x| println!("{x:?}"));
}

pub fn count_operations(config: &Config) {
  let data = read_data(config);
  println!("{}", data.len());
}
