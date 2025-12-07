// use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the problem set,
/// solve the problems and accumulate the answers.ingredient database,
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
    let _lines = rdr.lines();

    let total_splits: i64 = 0;

    // Display the grand total of problem answers
    //
    println!("The total beam splits is {}", total_splits);
    Ok(())
}

// test with example input
//
#[test]
fn given_example() {
    let expected: i64 = 1;
    let raw_input = " .......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
    .to_string();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    for line in lines {
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
    }
    let actual: i64 = 0;
    assert_eq!(expected, actual);
}
