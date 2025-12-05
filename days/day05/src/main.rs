use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the ingredient database,
/// identify and count the fresh ingredients. Output the
/// number of fresh ingredients.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

// Binary crate entry point
//
fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let rdr = BufReader::new(f);
    let lines = rdr.lines();

    let fresh_ingredient_count: u64 = 0;

    // Display the total number of fresh ingredients
    //
    println!(
        "The count of fresh ingredients is {}",
        fresh_ingredient_count
    );
    Ok(())
}
