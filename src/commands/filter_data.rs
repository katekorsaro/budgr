use crate::data::{Operation, Status};
use crate::Budgr;

pub fn filter_data(data: Vec<Operation>, args: &Budgr) -> Vec<Operation> {
  data
    .into_iter()
    // filter by id
    .filter(|x| {
      if let Some(id) = args.id {
        x.id == id
      } else {
        true
      }
    })
    // filter by delete
    .filter(|operation| {
      if args.deleted {
        operation.status == Status::Deleted
      } else {
        operation.status != Status::Deleted
      }
    })
    // filter by account
    .filter(|operation| {
      if let Some(account) = &args.account {
        operation
          .account
          .clone()
          .unwrap_or_default()
          .to_lowercase()
          .contains(&account.to_lowercase())
      } else {
        true
      }
    })
    // filter by dates
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
