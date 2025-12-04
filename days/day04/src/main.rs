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

    // For the cell at (row_idx, col_idx), count the neighboring
    // cells that contain rolls.
    //
    // Returns None if cell is not within the grid; returns
    // Some(count) where count is the number of neighboring cells
    // containing a roll of paper.
    //
    fn count_neighboring_rolls(
        &self,
        row_idx: u32,
        col_idx: u32,
    ) -> Option<u32> {
        // check whether cell is within the grid
        //
        if (row_idx >= self.row_count) || (col_idx >= self.col_count) {
            return None;
        }
        let mut roll_count: u32 = 0;
        //
        // look at neighbors above
        //
        if row_idx > 0 {
            let ridx: u32 = row_idx - 1;
            let cidx_from: u32 =
                if col_idx > 0 { col_idx - 1 } else { col_idx };
            let cidx_to: u32 = if col_idx == (self.col_count - 1) {
                col_idx
            } else {
                col_idx + 1
            };
            for cidx in cidx_from..=cidx_to {
                if self.has_roll(&ridx, &cidx) {
                    roll_count += 1;
                }
            }
        }
        //
        // look at neighbors on each side
        //
        let ridx: u32 = row_idx;
        if col_idx > 0 {
            let cidx: u32 = col_idx - 1;
            if self.has_roll(&ridx, &cidx) {
                roll_count += 1;
            }
        }
        if col_idx < (self.col_count - 1) {
            let cidx: u32 = col_idx + 1;
            if self.has_roll(&ridx, &cidx) {
                roll_count += 1;
            }
        }
        //
        // look at neighbors below
        //
        if row_idx < (self.row_count - 1) {
            let ridx: u32 = row_idx + 1;
            let cidx_from: u32 =
                if col_idx > 0 { col_idx - 1 } else { col_idx };
            let cidx_to: u32 = if col_idx == (self.col_count - 1) {
                col_idx
            } else {
                col_idx + 1
            };
            for cidx in cidx_from..=cidx_to {
                if self.has_roll(&ridx, &cidx) {
                    roll_count += 1;
                }
            }
        }
        //
        // Get out
        //
        Some(roll_count)
    }

    // Get the cell value
    //
    // Will panic if cell coordinates are not within the grid.
    //
    fn has_roll(&self, row_idx: &u32, col_idx: &u32) -> bool {
        let row = self.rows.get(row_idx).unwrap();
        *row.get(col_idx).unwrap()
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
    let mut accessible_rolls: u32 = 0;
    for ridx in 0..grid.row_count {
        for cidx in 0..grid.col_count {
            if grid.has_roll(&ridx, &cidx) {
                if 4 > grid.count_neighboring_rolls(ridx, cidx).unwrap()
                {
                    accessible_rolls += 1;
                }
            }
        }
    }

    println!(
        "The number of rolls accessible by a forklift is {}",
        accessible_rolls
    );
    Ok(())
}

// PaperRollGrid test helpers
//
// ..@@...@
// @..@@...
// .@..@@..
// ...@..@@
// @...@..@
// @@...@..
//
#[cfg(test)]
fn testhelper_make_grid01() -> PaperRollGrid {
    let mut grid = PaperRollGrid::new();
    let ss = String::from("..@@...@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("@..@@...");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from(".@..@@..");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("...@..@@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("@...@..@");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    let ss = String::from("@@...@..");
    let s: &str = ss.as_str();
    let _rolls = grid.add_next_row(s).unwrap();
    grid
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

// ..@@...@
// @..@@...
// .@..@@..
// ...@..@@
// @...@..@
// @@...@..
#[test]
fn count_neighbors_grid01_r0c0() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 1;
    let actual_count = grid.count_neighboring_rolls(0, 0).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r0c1() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(0, 1).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r0c6() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 1;
    let actual_count = grid.count_neighboring_rolls(0, 6).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r0c7() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 0;
    let actual_count = grid.count_neighboring_rolls(0, 7).unwrap();
    assert_eq!(expected_count, actual_count);
}
// ..@@...@
// @..@@...
// .@..@@..
// ...@..@@
// @...@..@
// @@...@..
#[test]
#[should_panic]
fn count_neighbors_grid01_r6c0() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let actual_count = grid.count_neighboring_rolls(6, 0).unwrap();
}
#[test]
#[should_panic]
fn count_neighbors_grid01_r0c8() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let actual_count = grid.count_neighboring_rolls(0, 8).unwrap();
}
#[test]
#[should_panic]
fn count_neighbors_grid01_r6c8() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let actual_count = grid.count_neighboring_rolls(6, 8).unwrap();
}

#[test]
fn count_neighbors_grid01_r1c0() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 1;
    let actual_count = grid.count_neighboring_rolls(1, 0).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r1c1() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 3;
    let actual_count = grid.count_neighboring_rolls(1, 1).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r1c6() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(1, 6).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r1c7() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 1;
    let actual_count = grid.count_neighboring_rolls(1, 7).unwrap();
    assert_eq!(expected_count, actual_count);
}
// ..@@...@
// @..@@...
// .@..@@..
// ...@..@@
// @...@..@
// @@...@..
#[test]
fn count_neighbors_grid01_r4c0() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(4, 0).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r4c1() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 3;
    let actual_count = grid.count_neighboring_rolls(4, 1).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r4c6() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 4;
    let actual_count = grid.count_neighboring_rolls(4, 6).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r4c7() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(4, 7).unwrap();
    assert_eq!(expected_count, actual_count);
}
// ..@@...@
// @..@@...
// .@..@@..
// ...@..@@
// @...@..@
// @@...@..
#[test]
fn count_neighbors_grid01_r5c0() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(5, 0).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r5c1() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(5, 1).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r5c6() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 2;
    let actual_count = grid.count_neighboring_rolls(5, 6).unwrap();
    assert_eq!(expected_count, actual_count);
}
#[test]
fn count_neighbors_grid01_r5c7() {
    let grid: PaperRollGrid = testhelper_make_grid01();
    let expected_count: u32 = 1;
    let actual_count = grid.count_neighboring_rolls(5, 7).unwrap();
    assert_eq!(expected_count, actual_count);
}
