use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

/// Given input file containing the comma-separated list of product ID
/// ranges, determine the sum of all invalid product IDs.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing product ID ranges
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let rdr = BufReader::new(f);
    let lines = rdr.lines();

    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
    }
    Ok(())
}
