mod add_operation;
mod count_operations;
mod filter_data;
mod list_operations;
mod modify_operations;
mod print_pretty;
mod print_version;

pub use add_operation::*;
pub use count_operations::*;
pub use list_operations::*;
pub use modify_operations::*;
pub use print_version::*;

use crate::cli::Format;
use crate::data::Data;

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
