use std::error::Error;
use std::io;
use std::path::Path;

use clap::{App, Arg};

mod config;
use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(option_env!("CARGO_PKG_NAME").unwrap())
        .version(option_env!("CARGO_PKG_VERSION").unwrap())
        .author(option_env!("CARGO_PKG_AUTHORS").unwrap())
        .about("Configuration file example")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Configuration file name")
                .required(true),
        )
        .arg(
            Arg::with_name("genconfig")
                .long("genconfig")
                .help("Generate a template configuration file"),
        )
        .get_matches();

    let config_filename = matches.value_of("config").unwrap();
    let config_path = Path::new(config_filename);
    if matches.is_present("genconfig") {
        if config_path.exists() {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Config file '{}' already exists", config_path.display()),
            ))?;
        } else {
            Config::default().write_to_path(&config_path)?;
        }
    } else {
        let config = Config::read_from_path(&config_path)?;
        println!("{:?}", config);
    }

    Ok(())
}
