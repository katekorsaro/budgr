use std::fs;
use crate::Config;

pub fn list_operations(config: &Config) {
  let data = fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap();
  data.lines().for_each(|line| println!("{line}"));
}

pub fn count_operations(config: &Config) {
  let data = fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap();
  let count = data.lines().count() - 1;

  println!("{count}");
}

