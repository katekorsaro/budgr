use crate::cli::Sort;
use crate::commands::filter_data::filter_data;
use crate::commands::print_pretty::print_pretty;
use crate::commands::print_raw::print_raw;
use crate::commands::Format;
use crate::config::Config;
use crate::data::read_data;
use crate::Budgr;
use crate::Command::List;

pub fn list_operations(config: &Config, args: &Budgr) {
  if let Some(List {
    output_format,
    date_sort,
    include_id,
    width,
    ..
  }) = &args.command
  {
    let data = read_data(config);
    let mut data = filter_data(data, args);
    match date_sort {
      Some(Sort::Desc) => data.sort_by(|a, b| b.date.cmp(&a.date)),
      Some(Sort::Asc) => data.sort_by(|a, b| a.date.cmp(&b.date)),
      _ => unreachable!("Desc is set be default value"),
    }
    match output_format {
      Some(Format::Raw) => print_raw(data),
      Some(Format::Pretty) => print_pretty(data, *include_id, *width),
      _ => unreachable!(),
    }
  }
}
