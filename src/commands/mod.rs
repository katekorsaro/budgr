mod list_operations;
mod count_operations;
mod print_version;
mod modify_operations;

pub use list_operations::*;
pub use count_operations::*;
pub use print_version::*;
pub use modify_operations::*;

use crate::cli::{Budgr, Command, Format};
use crate::data::{Data};
use crate::Config;
use prettytable::{row, Table};
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;

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

pub fn add_operation(config: &Config, args: &Budgr) {
  if let Some(Command::Add { date, note, amount }) = &args.command {
    let mut operation = Data {
      date: *date,
      note: String::from(note),
      amount: *amount,
      ..Data::default()
    };
    let mut rng = thread_rng();
    loop {
      let id: u32 = rng.gen();
      let filename = format!("{}{}.{:010}.bgr", config.data, date, id);
      let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .truncate(true)
        .open(&filename);
      if let Ok(mut file) = file {
        operation.id = id;
        let string_value = format!(
          "{}|{}|{}|{}|{}|{}|{}",
          operation.id,
          operation.date,
          operation.note,
          operation.amount,
          operation.account.clone().unwrap_or("".to_string()),
          operation.purpose.clone().unwrap_or("".to_string()),
          operation.goal.clone().unwrap_or("".to_string())
        );
        let _ = file.write_all(string_value.as_bytes());
        break;
      }
    }
  }
}
