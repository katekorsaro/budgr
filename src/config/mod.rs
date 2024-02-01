use std::fs;

#[derive(Debug, Default)]
pub struct Config {
  pub data: String,
}

pub fn read_configuration(filename: &str) -> Config {
  let mut retvalue = Config::default();

  // try to read configuration file
  let config = fs::read_to_string(filename).unwrap();
  config
    .lines()
    .map(|line| {
      let mut parts = line.split('=');
      (parts.next().unwrap(), parts.next())
    })
    //handling single config keys
    .for_each(|kv_pair| match kv_pair.0 {
      "budgr.data" => {
        if let Some(value) = kv_pair.1 {
          retvalue.data = String::from(value);
        }
      }
      _ => unreachable!("All configurations should be covered for now."),
    });

  retvalue
}
