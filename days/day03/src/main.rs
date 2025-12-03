use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
// use utf8_chars::BufReadCharsExt;

/// Given input file containing the battery bank specs,
/// determine the max joltage for each bank and the
/// total overall joltage.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

// Elevator battery bank info.
//
struct BatteryBank {
    // joltage rating by battery id (the index, not the position)
    //
    joltage_by_idx: BTreeMap<u32, u32>,
    // index of the first battery with the largest joltage
    //
    idx_1st_largest: u32,
    // index of the first battery with the next largest joltage
    // (which could be as much joltage as the first largest battery)
    //
    idx_2nd_largest: u32,
}

// methods and associated methods for the BatteryBank struct
//
impl BatteryBank {
    fn new(spec: &str) -> Self {
        let mut jbi: BTreeMap<u32, u32> = BTreeMap::new();
        let mut i1l: u32 = 0;
        let mut i2l: u32 = 0;
        let mut j1l: u32 = 0;
        let mut j2l: u32 = 0;
        for (ii, c) in spec.chars().enumerate() {
            if !c.is_digit(10) {
                break;
            }
            let i = ii.try_into().unwrap();
            let jj = c.to_digit(10).unwrap();
            let j = jj.try_into().unwrap();
            jbi.insert(i, j);
            if j > j1l {
                j2l = j1l;
                i2l = i1l;
                j1l = j;
                i1l = i;
            } else if j > j2l {
                j2l = j;
                i2l = i;
            }
        }
        BatteryBank {
            joltage_by_idx: jbi,
            idx_1st_largest: i1l,
            idx_2nd_largest: i2l,
        }
    }

    fn max_joltage(&self) -> Option<u32> {
        // if we didn't find two batteries, return None
        // because the battery bank spec must have been
        // invalid.
        //
        if self.idx_1st_largest == self.idx_2nd_largest {
            return None;
        }
        let maxj: u32;
        if self.idx_1st_largest < self.idx_2nd_largest {
            maxj =
                self.joltage_by_idx.get(&self.idx_1st_largest).unwrap()
                    * 10
                    + self
                        .joltage_by_idx
                        .get(&self.idx_2nd_largest)
                        .unwrap();
        } else {
            maxj =
                self.joltage_by_idx.get(&self.idx_2nd_largest).unwrap()
                    * 10
                    + self
                        .joltage_by_idx
                        .get(&self.idx_1st_largest)
                        .unwrap();
        }
        Some(maxj)
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

    // determine the max joltage for each bank
    //
    let mut joltages: Vec<u32> = Vec::new();
    let mut line_num = 0;
    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        line_num += 1;
        let battery_bank = BatteryBank::new(line.trim());
        joltages.push(battery_bank.max_joltage().unwrap());
    }

    // add up the max joltage for each bank
    //
    let mut joltage_accum: u32 = 0;
    for joltage in joltages {
        joltage_accum += joltage;
    }
    println!("The total joltage is {}.", joltage_accum);
    Ok(())
}

// BatteryBank tests
//
#[test]
fn check_descending() {
    let ss = String::from("987654321");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u32 = 98;
    match bb.max_joltage() {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_ascending() {
    let ss = String::from("123456789");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u32 = 89;
    match bb.max_joltage() {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_all_ones() {
    let ss = String::from("111111111");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u32 = 11;
    match bb.max_joltage() {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}
