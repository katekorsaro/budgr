use crate::cli::{Budgr, Format};
use crate::data::{read_data, Data};
use crate::Config;
use prettytable::{row, Table};

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
  match args.output_format {
    Some(Format::Raw) => print_raw(data),
    Some(Format::Pretty) => print_pretty(data),
    _ => unreachable!(),
  }
}

fn print_pretty(data: Vec<Data>) {
  let mut table = Table::new();
  table.add_row(row![
    "Id", "Date", "Note", "Amount", "Account", "Purpose", "Goal"
  ]);
  data.into_iter().for_each(|operation| {
    table.add_row(row![
      operation.id.to_string(),
      operation.date.to_string(),
      operation.note,
      r->operation.amount.to_string(),
      operation.account.unwrap_or("-".to_string()),
      operation.purpose.unwrap_or("-".to_string()),
      operation.goal.unwrap_or("-".to_string()),
    ]);
  });
  table.printstd();
}
fn print_raw(data: Vec<Data>) {
  data.into_iter().for_each(|operation| {
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
  });
}
pub fn count_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  println!("{}", data.len());
}

pub fn print_version() {
  let version = env!("CARGO_PKG_VERSION");
  let name = env!("CARGO_PKG_NAME");
  println!("{name}: {version}");
}
