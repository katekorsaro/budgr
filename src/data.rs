use crate::Config;
use std::fs;

pub fn read_data(config: &Config) -> String {
  fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap()
}
