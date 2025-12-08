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
    let lines = rdr.lines();

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
    }

    // Display the grand total of problem answers
    //
    let path_count: usize = 0;
    println!("The path count is {}", path_count);
    Ok(())
}

// test with example input
//
#[test]
fn given_example_part1() {
    let expected_product: usize = 40;
    let raw_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        .to_string();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    for line in lines {
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
    }
    let actual_product: usize = 0;
    assert_eq!(expected_product, actual_product);
}
