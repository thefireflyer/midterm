use std::{
    io::{stdin, stdout, IsTerminal, Read},
    path::PathBuf,
    process::Command,
};

use anyhow::Result;
use clap::Parser;
use cursive::backends::crossterm::crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use spell_checker::SpellChecker;

use crate::interactive::start_interactive;

///////////////////////////////////////////////////////////////////////////////

mod cli;
mod interactive;

//cargo run -- --path '/usr/share/myspell/dicts/en_US-large.dic

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let args = cli::Config::parse();

    let source = args.path.unwrap_or_else(|| find_default().unwrap());

    let mut spell_checker = SpellChecker::from_file(&source)?;

    println!();
    println!("{:-^80}", "");
    println!();

    if stdin().is_terminal() {
        start_interactive(spell_checker)
    } else {
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(target_os = "linux")]
fn find_default() -> Result<PathBuf> {
    use std::{fs, process::Stdio};

    println!("No dictionary provided, searching default paths...");

    let output = Command::new("fd")
        .arg("-e")
        .arg("dic")
        .current_dir("/")
        .stdout(Stdio::piped())
        .output()
        .expect("Process failed to execute");

    let out = String::from_utf8(output.stdout)?;

    let paths: Vec<&str> = out.split('\n').collect();

    println!("Found {} possible sources", paths.len());
    println!();

    for path in paths {
        if path.contains("en_US")
            || path.contains("en_GB")
            || path.contains("standard")
            || path.contains("default")
        {
            let path = "/".to_owned() + path;

            println!("Checking {}", path);

            match fs::read_to_string(path.clone()) {
                Ok(contents) => {
                    if contents.contains("probably") {
                        println!("Using {}", path.clone());
                        return Ok(PathBuf::from(path));
                    }
                }
                Err(err) => println!("Failed to open {} because {}", path, err),
            };
        }
    }

    Ok(PathBuf::new())
}

///////////////////////////////////////////////////////////////////////////////
