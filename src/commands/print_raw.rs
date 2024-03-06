use crate::data::Operation;

pub fn print_raw(data: Vec<Operation>) {
    data.into_iter().for_each(|operation| {
        println!("{}", operation.to_raw_string());
    });
}
