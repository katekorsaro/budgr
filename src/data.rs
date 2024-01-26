use crate::Config;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum Status {
  Active,
  Deleted,
}

impl FromStr for Status {
  type Err = String;
  fn from_str(value: &str) -> Result<Self, String> {
    match value {
      "active" => Ok(Status::Active),
      "deleted" => Ok(Status::Deleted),
      _ => Err(format!("value '{}' not mapped", value)),
    }
  }
}

impl ToString for Status {
  fn to_string(&self) -> String {
    match self {
      Status::Active => String::from("active"),
      Status::Deleted => String::from("deleted"),
    }
  }
}

#[derive(Debug)]
pub struct Data {
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

impl Data {
  pub fn default() -> Self {
    Data {
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

    Ok(Data {
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
        .map(|value| value.replace("\n", "").to_string()),
      path: String::from(""),
    })
  }

  pub fn to_raw_string(&self) -> String {
    format!(
      "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
      self.id,
      self.status.to_string(),
      self.creation_date,
      self.modification_date.clone().unwrap_or(String::new()),
      self.date,
      self.note,
      self.amount,
      self.account.clone().unwrap_or(String::new()),
      self.purpose.clone().unwrap_or(String::new()),
      self.goal.clone().unwrap_or(String::new()),
    )
  }
}

pub fn read_data(config: &Config) -> Vec<Data> {
  // get all files in dir
  let path = Path::new(&config.data);
  let files = fs::read_dir(path).unwrap();
  files
    .map(|file| file.unwrap().path())
    .filter(|path| path.extension().unwrap() == "bgr")
    .map(|path| (fs::read_to_string(path.clone()).unwrap(), path))
    .map(|(string_value, path)| {
      let mut data = Data::from_string(&string_value).unwrap();
      data.path = String::from(path.to_str().unwrap());
      data
    })
    .collect::<Vec<Data>>()
}

#[test]
fn parse_data() {
  let input: String = String::from(
    "1|active|2024-01-01 10:55:35|2024-01-02 12:55:35|20240101|Note|10000|bank|purpose|goal",
  );
  let data: Data = Data::from_string(&input).unwrap();
  assert_eq!(data.id, 1);
  assert_eq!(data.creation_date, String::from("2024-01-01 10:55:35"));
  assert_eq!(
    data.modification_date,
    Some(String::from("2024-01-02 12:55:35"))
  );
  assert_eq!(data.date, 20240101);
  assert_eq!(data.note, "Note".to_string());
  assert_eq!(data.amount, 10000);
  assert_eq!(data.account, Some(String::from("bank")));
  assert_eq!(data.purpose, Some(String::from("purpose")));
  assert_eq!(data.goal, Some(String::from("goal")));
}
