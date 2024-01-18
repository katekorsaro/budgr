use clap::{Parser, Subcommand};
use std::{env, fs};

#[derive(Parser)]
#[command()]
struct Budgr {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all operations
    List,
}

#[derive(Debug, Default)]
struct Config {
    pub data: String,
}

fn main() {
    // read env variable to get configuration
    let budgrrc = env::vars().find(|(k, _)| k == "BUDGRRC");

    // store env variable into an actual variable
    let budgrrc = match budgrrc {
        Some((_, value)) => value,
        None => String::from(".//.budgrrc"),
    };

    let config = read_configuration(&budgrrc);
    println!("{config:?}");

    let _cmd = Budgr::parse();
}

fn read_configuration(filename: &str) -> Config {
    let mut retvalue = Config::default();

    // try to read configuration file
    let config = fs::read_to_string(filename).unwrap();
    config
        .lines()
        .map(|x| {
            let mut parts = x.split('=');
            (parts.next().unwrap(), parts.next())
        })
        //handling single config keys
        .for_each(|x| match x.0 {
            "budgr.data" => {
                if let Some(value) = x.1 {
                    retvalue.data = String::from(value);
                }
            }
            _ => unreachable!("All configurations should be covered for now."),
        });

    retvalue
}
