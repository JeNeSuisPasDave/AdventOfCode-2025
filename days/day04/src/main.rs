use std::collections::BTreeMap;
use std::fmt;
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

#[derive(Debug)]
enum PaperRollGridError {
    InputRowWrongLength,
    InvalidInputCharacter,
}

impl fmt::Display for PaperRollGridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaperRollGridError::InputRowWrongLength => {
                write!(
                    f,
                    "Cannot add row with a different number of columns than existing rows"
                )
            }
            PaperRollGridError::InvalidInputCharacter => {
                write!(f, "Invalid grid specification character")
            }
        }
    }
}

impl std::error::Error for PaperRollGridError {}

struct PaperRollGrid {
    // A collection of rows indexed by zero-based row number.
    // Each row is a collection of cells indexed by zero-based
    // column number. If the cell is true, it is occupied by
    // a paper roll.
    //
    rows: BTreeMap<u32, BTreeMap<u32, bool>>,
    row_count: u32,
    col_count: u32,
}

impl PaperRollGrid {
    // constructor
    //
    fn new() -> Self {
        let g: BTreeMap<u32, BTreeMap<u32, bool>> = BTreeMap::new();
        PaperRollGrid {
            rows: g,
            row_count: 0,
            col_count: 0,
        }
    }

    // add another row to the grid and return the number of
    // rolls found in the specification string.
    //
    fn add_next_row(
        &mut self,
        row_spec: &str,
    ) -> Result<u32, PaperRollGridError> {
        let mut roll_count: u32 = 0;
        if self.rows.len() == 0 {
            let mut row: BTreeMap<u32, bool> = BTreeMap::new();
            for (ii, c) in row_spec.chars().enumerate() {
                let i = ii.try_into().unwrap();
                let contains_roll = match c {
                    '.' => false,
                    '@' => {
                        roll_count += 1;
                        true
                    }
                    _ => {
                        return Err(
                            PaperRollGridError::InvalidInputCharacter,
                        );
                    }
                };
                row.insert(i, contains_roll);
            }
            self.col_count = row.len().try_into().unwrap();
            self.rows.insert(self.row_count, row);
            self.row_count += 1;
        } else {
            let mut row: BTreeMap<u32, bool> = BTreeMap::new();
            for (ii, c) in row_spec.chars().enumerate() {
                let i = ii.try_into().unwrap();
                let contains_roll = match c {
                    '.' => false,
                    '@' => {
                        roll_count += 1;
                        true
                    }
                    _ => {
                        return Err(
                            PaperRollGridError::InvalidInputCharacter,
                        );
                    }
                };
                row.insert(i, contains_roll);
            }
            let rl: u32 = row.len().try_into().unwrap();
            if self.col_count != rl {
                return Err(PaperRollGridError::InputRowWrongLength);
            }
            self.rows.insert(self.row_count, row);
            self.row_count += 1;
        }
        Ok(roll_count)
    }
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

    let mut grid = PaperRollGrid::new();
    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        let line = line.trim();
        _ = grid.add_next_row(line)?;
    }
    let _ = grid.row_count;

    println!(
        "The number of rolls accessible by a forklift is {}",
        "unknown"
    );
    Ok(())
}

// PaperRollGrid tests
//

#[test]
#[should_panic]
fn has_invalid_spec_char() {
    let mut grid = PaperRollGrid::new();
    let ss = String::from("..@@+...@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
}

#[test]
fn adding_1st_row() {
    let mut grid = PaperRollGrid::new();
    let ss = String::from("..@@...@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    assert_eq!(1, grid.row_count);
    assert_eq!(8, grid.col_count);
}

#[test]
fn adding_several_rows() {
    let mut grid = PaperRollGrid::new();
    let ss = String::from("..@@...@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    assert_eq!(1, grid.row_count);
    assert_eq!(8, grid.col_count);
    let ss = String::from("@..@@..@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("..@@@@.@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
}

#[test]
#[should_panic]
fn adding_row_of_different_length() {
    let mut grid = PaperRollGrid::new();
    let ss = String::from("..@@...@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    assert_eq!(1, grid.row_count);
    assert_eq!(8, grid.col_count);
    let ss = String::from("@..@@..@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("..@@...@@@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
}
