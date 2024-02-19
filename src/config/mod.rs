use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  pub data: String,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      data: String::from("./data/"),
    }
  }
}

pub fn read_configuration() -> Config {
  let config: Config = confy::load_path("./budgr.toml").unwrap();
  config
}
