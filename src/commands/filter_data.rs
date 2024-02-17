use crate::data::{Operation, Status};
use crate::Budgr;

pub fn filter_data(data: Vec<Operation>, args: &Budgr) -> Vec<Operation> {
  // considering only a subset of operations
  let operation_count = if let Some(operation_count) = &args.operation_count {
    *operation_count
  } else {
    usize::MAX
  };
  let mut data = data;
  data.sort_by(|a, b| a.date.cmp(&b.date));
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
    // filter by purpose
    .filter(|operation| {
      if let Some(purpose) = &args.purpose {
        operation.purpose.clone().unwrap() == String::from(purpose)
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
    // filter by amount
    .filter(|operation| {
      if let Some(amount_gt) = args.amount_greater_than {
        operation.amount >= amount_gt
      } else {
        true
      }
    })
    .filter(|operation| {
      if let Some(amount_lt) = args.amount_less_than {
        operation.amount <= amount_lt
      } else {
        true
      }
    })
    // filter by goal
    .filter(|operation| {
      if let Some(goal) = &args.goal {
        operation
          .goal
          .clone()
          .unwrap()
          .to_lowercase()
          .contains(&goal.to_lowercase())
      } else {
        true
      }
    })
    // filter by note
    .filter(|operation| {
      if let Some(note) = &args.note {
        operation
          .note
          .clone()
          .to_lowercase()
          .contains(&note.to_lowercase())
      } else {
        true
      }
    })
    .take(operation_count)
    .collect()
}
