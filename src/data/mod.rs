mod status;

use crate::Config;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub use status::*;

#[derive(Debug)]
pub struct Operation {
  pub account: Option<String>,
  pub amount: u32,
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

  fn from_string(string_value: &str) -> Result<Self, String> {
    let mut parts = string_value.split('|');

    Ok(Operation {
      id: parts.next().unwrap().parse::<u32>().unwrap(),
      status: Status::from_str(parts.next().unwrap()).unwrap(),
      creation_date: String::from(parts.next().unwrap()),
      modification_date: parts.next().map(|value| value.to_string()),
      date: parts.next().unwrap().parse::<u32>().unwrap(),
      note: String::from(parts.next().unwrap()),
      amount: parts.next().unwrap().parse::<u32>().unwrap(),
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

pub fn read_data(config: &Config) -> Vec<Operation> {
  // get all files in dir
  let path = Path::new(&config.data);
  let files = fs::read_dir(path).unwrap();
  files
    .map(|file| file.unwrap().path())
    .filter(|path| path.extension().unwrap() == "bgr")
    .map(|path| (fs::read_to_string(path.clone()).unwrap(), path))
    .map(|(string_value, path)| {
      let mut data = Operation::from_string(&string_value).unwrap();
      data.path = String::from(path.to_str().unwrap());
      data
    })
    .collect::<Vec<Operation>>()
}

#[test]
fn parse_data() {
  let input: String = String::from(
    "1|active|2024-01-01 10:55:35|2024-01-02 12:55:35|20240101|Note|10000|bank|purpose|goal",
  );
  let operation: Operation = Operation::from_string(&input).unwrap();
  assert_eq!(operation.id, 1);
  assert_eq!(operation.creation_date, String::from("2024-01-01 10:55:35"));
  assert_eq!(
    operation.modification_date,
    Some(String::from("2024-01-02 12:55:35"))
  );
  assert_eq!(operation.date, 20240101);
  assert_eq!(operation.note, "Note".to_string());
  assert_eq!(operation.amount, 10000);
  assert_eq!(operation.account, Some(String::from("bank")));
  assert_eq!(operation.purpose, Some(String::from("purpose")));
  assert_eq!(operation.goal, Some(String::from("goal")));
}