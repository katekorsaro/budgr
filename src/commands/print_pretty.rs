use crate::data::Operation;
use prettytable::{row, Table};

pub fn print_pretty(data: Vec<Operation>) {
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
