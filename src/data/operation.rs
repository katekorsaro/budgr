use crate::data::Status;
use std::str::FromStr;

#[derive(Debug)]
pub struct Operation {
  pub account: Option<String>,
  pub amount: i32,
  pub creation_date: String,
  pub date: u32,
  pub goal: Option<String>,
  pub id: u32,
  pub modification_date: Option<String>,
  pub note: String,
  pub path: String,
  pub purpose: Option<String>,
  pub status: Status,
}

impl Operation {
  pub fn default() -> Self {
    Operation {
      account: None,
      amount: 0,
      creation_date: String::new(),
      date: 0,
      goal: None,
      id: 0,
      modification_date: None,
      note: String::new(),
      path: String::new(),
      purpose: None,
      status: Status::Active,
    }
  }

  pub fn from_string(string_value: &str) -> Result<Self, String> {
    let mut parts = string_value.split('|');

    Ok(Operation {
      id: parts.next().unwrap().parse::<u32>().unwrap(),
      status: Status::from_str(parts.next().unwrap()).unwrap(),
      creation_date: String::from(parts.next().unwrap()),
      modification_date: parts.next().map(|value| value.to_string()),
      date: parts.next().unwrap().parse::<u32>().unwrap(),
      note: String::from(parts.next().unwrap()),
      amount: parts.next().unwrap().parse::<i32>().unwrap(),
      account: parts.next().map(|value| value.to_string()),
      purpose: parts.next().map(|value| value.to_string()),
      goal: parts
        .next()
        .map(|value| value.replace('\n', "").to_string()),
      path: String::from(""),
    })
  }

  pub fn to_raw_string(&self) -> String {
    format!(
      "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
      self.id,
      self.status.to_string(),
      self.creation_date,
      self.modification_date.clone().unwrap_or_default(),
      self.date,
      self.note,
      self.amount,
      self.account.clone().unwrap_or_default(),
      self.purpose.clone().unwrap_or_default(),
      self.goal.clone().unwrap_or_default(),
    )
  }
}
