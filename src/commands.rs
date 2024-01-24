use crate::cli::Budgr;
use crate::data::{read_data, Data};
use crate::Config;

fn filter_data(data: Vec<Data>, args: &Budgr) -> Vec<Data> {
  data
    .into_iter()
    .filter(|operation| {
      if let Some(from_date) = args.from {
        operation.date >= from_date
      } else {
        true
      }
    })
    .filter(|operation| {
      if let Some(to_date) = args.to {
        operation.date <= to_date
      } else {
        true
      }
    })
    .collect()
}
pub fn list_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  data
    .into_iter()
    .for_each(|operation| println!("{operation:?}"));
}

pub fn count_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  println!("{}", data.len());
}
