use crate::cli::{Budgr, Format};
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
    .for_each(|operation| match args.output_format {
      Some(Format::Raw) => print_raw(operation),
      _ => unreachable!("There are no other options..."),
    });
}

fn print_raw(operation: Data) {
  println!(
    "{}|{}|{}|{}|{}|{}|{}",
    operation.id,
    operation.date,
    operation.note,
    operation.amount,
    operation.account.unwrap_or("-".to_string()),
    operation.purpose.unwrap_or("-".to_string()),
    operation.goal.unwrap_or("-".to_string())
  );
}
pub fn count_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  println!("{}", data.len());
}
