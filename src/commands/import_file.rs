use crate::commands::write_operation::write_operation;
use crate::data::Operation;
use crate::Budgr;
use crate::Command;
use crate::Config;
use std::fs;

pub fn import_file(config: &Config, args: &Budgr) {
  if let Some(Command::Import { filename }) = &args.command {
    let data = fs::read_to_string(filename).unwrap();
    data
      .lines()
      .map(|line| line.split('|'))
      .map(|mut parts| Operation {
        date: parts.next().unwrap().parse::<u32>().unwrap(),
        note: String::from(parts.next().unwrap()),
        amount: parts.next().unwrap().parse::<u32>().unwrap(),
        ..Operation::default()
      })
      .for_each(|operation| {
        write_operation(operation, None, &config);
      });
  }
}
