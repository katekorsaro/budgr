use crate::data::*;
use crate::commands::filter_data::*;
use crate::*;

pub fn aggregate (config: &Config, args: &Budgr) {
    let data = read_data(config);
    let data = filter_data(data, args);
    let sum = data.into_iter()
        .fold(0, |acc, operation: Operation| acc + operation.amount);
    println!("{sum}");
}
