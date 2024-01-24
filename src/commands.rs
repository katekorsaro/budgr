use crate::cli::Budgr;
use crate::data::read_data;
use crate::Config;

pub fn list_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
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
    .for_each(|x| println!("{x:?}"));
}

pub fn count_operations(config: &Config) {
  let data = read_data(config);
  println!("{}", data.len());
}
