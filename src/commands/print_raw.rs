use crate::data::Data;

pub fn print_raw(data: Vec<Data>) {
  data.into_iter().for_each(|operation| {
    println!(
        "{}", operation.to_raw_string()
    );
  });
}
