use crate::commands::filter_data::filter_data;
use crate::config::Config;
use crate::data::read_data;
use crate::Budgr;

pub fn count_operations(config: &Config, args: &Budgr) {
    let data = read_data(config);
    let data = filter_data(data, args);
    println!("{}", data.len());
}
