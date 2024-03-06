use crate::data::*;
use crate::Config;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_operation(mut operation: Operation, id: Option<u32>, config: &Config) -> u32 {
    let mut rng = thread_rng();
    let mut force = false;
    loop {
        let id = if let Some(id) = id {
            force = true;
            id
        } else {
            rng.gen::<u32>()
        };
        let filename = format!("{}{}.{:010}.bgr", config.data, operation.date, id);
        let file = OpenOptions::new()
            .create(force) // in case of fixed id
            .create_new(!force) // in case of rnd id
            .write(true)
            .truncate(true)
            .open(&filename);
        if let Ok(mut file) = file {
            operation.id = id;
            let string_value = operation.to_raw_string();
            let _ = file.write_all(string_value.as_bytes());
            return id;
        }
    }
}
