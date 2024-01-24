use crate::Config;
use std::fs;

#[derive(Debug)]
pub struct Data {
  pub id: u32,
  pub date: u32,
  pub note: String,
  pub amount: i32,
  pub account: Option<String>,
  pub purpose: Option<String>,
  pub goal: Option<String>,
}

impl Data {
  fn from_string(string_value: &str) -> Result<Self, String> {
    let mut parts = string_value.split('|');

    Ok(Data {
      id: parts.next().unwrap().parse::<u32>().unwrap(),
      date: parts.next().unwrap().parse::<u32>().unwrap(),
      note: String::from(parts.next().unwrap()),
      amount: parts.next().unwrap().parse::<i32>().unwrap(),
      account: parts.next().map(|value| value.to_string()),
      purpose: parts.next().map(|value| value.to_string()),
      goal: parts.next().map(|value| value.to_string()),
    })
  }
}
pub fn read_data(config: &Config) -> Vec<Data> {
  let data = fs::read_to_string(format!("{}//data.tsv", config.data)).unwrap();
  data
    .lines()
    .skip(1)
    .map(|line| Data::from_string(line).unwrap())
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