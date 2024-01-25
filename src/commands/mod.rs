mod add_operation;
mod count_operations;
mod list_operations;
mod modify_operations;
mod print_version;

pub use add_operation::*;
pub use count_operations::*;
pub use list_operations::*;
pub use modify_operations::*;
pub use print_version::*;

use crate::cli::{Budgr, Format};
use crate::data::Data;
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

fn print_pretty(data: Vec<Data>) {
  let mut table = Table::new();
  table.add_row(row![
    "Id",
    "Date",
    "Note",
    "Amount ",
    "Account",
    "Purpose",
    "Goal"
  ]);
  data.into_iter().for_each(|operation| {
    table.add_row(row![
      operation.id.to_string(),
      format!("{}", prettify_date(operation.date)),
      operation.note,
      r->format!("{:.2}", (operation.amount as f32)/100_f32),
      operation.account.unwrap_or("".to_string()),
      operation.purpose.unwrap_or("".to_string()),
      operation.goal.unwrap_or("".to_string()),
    ]);
  });
  table.printstd();
}

fn prettify_date(date: u32) -> String {
  let mut retvalue = date.to_string();
  retvalue.insert(6, '/');
  retvalue.insert(4, '/');
  retvalue
}

fn print_raw(data: Vec<Data>) {
  data.into_iter().for_each(|operation| {
    println!(
      "{}|{}|{}|{}|{}|{}|{} (file: {})",
      operation.id,
      operation.date,
      operation.note,
      operation.amount,
      operation.account.unwrap_or("".to_string()),
      operation.purpose.unwrap_or("".to_string()),
      operation.goal.unwrap_or("".to_string()),
      operation.path
    );
  });
}
