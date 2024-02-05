use crate::Budgr;
use crate::Command;

pub fn import_file(args: &Budgr) {
  if let Some(Command::Import { filename }) = &args.command {
    println!("{filename:?}");
  }
}
