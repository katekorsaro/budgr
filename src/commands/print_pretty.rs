use crate::cli::Purpose;
use crate::data::Operation;
use colorize::AnsiColor;
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
    let purpose = colorized_purpose(&operation);
    let amount = colorized_amount(&operation);
    table.add_row(row![
      operation.id.to_string(),
      format!("{}", prettify_date(operation.date)),
      operation.note,
      r->format!("{:.2}", (operation.amount as f32)/100_f32),
      amount,
      purpose,
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

fn colorized_purpose(operation: &Operation) -> String {
  if let Some(ref operation_purpose) = operation.purpose {
    let operation_purpose = operation_purpose.parse::<Purpose>().unwrap();
    return match operation_purpose {
      Purpose::Need => operation.purpose.clone().unwrap().b_blue(),
      Purpose::YearlyNeed => operation.purpose.clone().unwrap().blue(),
      Purpose::Want => operation.purpose.clone().unwrap().b_yellow(),
      Purpose::YearlyWant => operation.purpose.clone().unwrap().yellow(),
      Purpose::Goal => operation.purpose.clone().unwrap().b_red(),
      Purpose::Income => operation.purpose.clone().unwrap().b_green(),
      _ => operation.purpose.clone().unwrap(),
    };
  } else {
    return String::new();
  };
}

fn colorized_amount(operation: &Operation) -> String {
  if let Some(ref operation_purpose) = operation.purpose {
    let operation_purpose = operation_purpose.parse::<Purpose>().unwrap();
    let amount = format!("{:.2}", (operation.amount as f32) / 100_f32);
    return match operation_purpose {
      Purpose::Income => amount.b_green(),
      _ => amount,
    };
  } else {
    return String::new();
  };
}
