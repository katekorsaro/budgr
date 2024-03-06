use crate::cli::Purpose;
use crate::data::Operation;
use colorize::AnsiColor;
use prettytable::{cell, row, Table};

pub fn print_pretty(data: Vec<Operation>, include_id: bool, width: Option<usize>) {
    let mut table = Table::new();
    let header = table.add_row(row![
      c->"Date 󰃭",
      c->"Note 󰎛",
      c->"Amount ",
      c->"Account ",
      c->"Purpose ",
      c->"Goal "
    ]);
    if include_id {
        header.insert_cell(0, cell!(c->"id"));
    }
    data.into_iter().for_each(|operation| {
        let purpose = colorized_purpose(&operation);
        let amount = colorized_amount(&operation);
        let note = if let Some(width) = width {
            let note = if operation.note.len() > width {
                String::from(&operation.note[..width])
            } else {
                operation.note.clone()
            };
            note
        } else {
            operation.note.clone()
        };
        let row = table.add_row(row![
          format!("{}", prettify_date(operation.date)),
          note,
          r->amount,
          operation.account.unwrap_or_default(),
          purpose,
          operation.goal.unwrap_or_default(),
        ]);
        if include_id {
            row.insert_cell(0, cell!(operation.id));
        }
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
        match operation_purpose {
            Purpose::Need => operation.purpose.clone().unwrap().b_blue(),
            Purpose::YearlyNeed => operation.purpose.clone().unwrap().blue(),
            Purpose::Want => operation.purpose.clone().unwrap().b_yellow(),
            Purpose::YearlyWant => operation.purpose.clone().unwrap().yellow(),
            Purpose::Goal => operation.purpose.clone().unwrap().b_red(),
            Purpose::Income => operation.purpose.clone().unwrap().b_green(),
            _ => operation.purpose.clone().unwrap(),
        }
    } else {
        String::new()
    }
}

fn colorized_amount(operation: &Operation) -> String {
    if let Some(ref operation_purpose) = operation.purpose {
        let operation_purpose = operation_purpose.parse::<Purpose>().unwrap();
        let amount = format!("{:.2}", (operation.amount as f32) / 100_f32);
        match operation_purpose {
            Purpose::Income => amount.b_green(),
            _ => amount,
        }
    } else {
        String::new()
    }
}
