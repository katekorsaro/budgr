use crate::Config;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Data {
  pub id: u32,
  pub date: u32,
  pub note: String,
  pub amount: i32,
  pub account: Option<String>,
  pub purpose: Option<String>,
  pub goal: Option<String>,
  pub path: String,
}

impl Data {
  pub fn default() -> Self {
    Data {
      id: 0,
      date: 0,
      note: String::new(),
      amount: 0,
      account: None,
      purpose: None,
      goal: None,
      path: String::new(),
    }
  }

  fn from_string(string_value: &str) -> Result<Self, String> {
    let mut parts = string_value.split('|');

    Ok(Data {
      id: parts.next().unwrap().parse::<u32>().unwrap(),
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
  let input: String = String::from("1|20240101|Note|10000|bank|purpose|goal");
  let data: Data = Data::from_string(&input).unwrap();
  assert_eq!(data.id, 1);
  assert_eq!(data.date, 20240101);
  assert_eq!(data.note, "Note".to_string());
  assert_eq!(data.amount, 10000);
  assert_eq!(data.account, Some(String::from("bank")));
  assert_eq!(data.purpose, Some(String::from("purpose")));
  assert_eq!(data.goal, Some(String::from("goal")));
}
