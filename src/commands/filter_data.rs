use crate::data::Operation;
use crate::Budgr;

pub fn filter_data(data: Vec<Operation>, args: &Budgr) -> Vec<Operation> {
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
