use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
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

