use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author="Aidan Beil",
    about="Spell checker", 
    long_about = Some("CS240 Midterm | Spell checker CLI")
)]

pub struct Config {
    /// Max number of spelling suggestion
    #[arg(short, long, default_value_t = 10)]
    pub suggestion_limit: u16,

    /// Path to dictionary (optional)
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
