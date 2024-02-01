use crate::config::Config;
use crate::data::Operation;
use std::fs;
use std::path::Path;

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
