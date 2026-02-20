mod config;

use config::Configuration;

use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Name of the configuration file
    #[arg(long, short, default_value = "config.yaml")]
    config: String,
    /// Write default configuration file
    #[arg(long, default_value_t = false)]
    write_config: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.write_config {
        Configuration::default().write_to_path(Path::new(&args.config))?;
        return Ok(());
    }

    let config = Configuration::build(Path::new(&args.config))?;
    println!("{config:?}");

    Ok(())
}
