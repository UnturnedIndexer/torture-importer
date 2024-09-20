use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the TOML file containing metadata for importing
    #[clap(short, long, value_name = "FILE")]
    pub config: PathBuf,
}
