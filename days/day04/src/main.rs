use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the paper roll grid,
/// output the number of paper rolls accessible by a forklift.
///
/// In this version, any roll with fewer than 4 neighbors can
/// be accessed by a forklift.
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

    println!(
        "The number of rolls accessible by a forklift is {}",
        "unknown"
    );
    Ok(())
}

// BatteryBank tests with 2 batteries
//

// #[test]
// fn check_all_ones() {
//     let ss = String::from("1111111111111111");
//     let s: &str = ss.as_str();
//     let bb = BatteryBank::new(s);
//     let expected: u64 = 11;
//     match bb.max_joltage(2) {
//         None => {
//             assert!(false, "FAILED to find max joltage")
//         }
//         Some(actual) => assert_eq!(expected, actual),
//     }
// }
