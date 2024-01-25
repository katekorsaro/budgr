use crate::commands::{print_pretty, print_raw, Format};
use crate::data::read_data;
use crate::Budgr;
use crate::Config;
use crate::commands::filter_data::filter_data;

pub fn list_operations(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  match args.output_format {
    Some(Format::Raw) => print_raw(data),
    Some(Format::Pretty) => print_pretty(data),
    _ => unreachable!(),
  }
}
