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
        // load an indexed map with the joltage values
        //
        let mut jbi: BTreeMap<u32, u32> = BTreeMap::new();
        for (ii, c) in spec.chars().enumerate() {
            if !c.is_digit(10) {
                break;
            }
            let i = ii.try_into().unwrap();
            let jj = c.to_digit(10).unwrap();
            let j = jj.try_into().unwrap();
            jbi.insert(i, j);
        }
        // find the max battery, but don't consider the last
        // battery.
        //
        let mut b_max: u32 = 0;
        let jbi_len: u32 = jbi.len().try_into().unwrap();
        let jbi_len_m1: u32 = jbi_len - 1;
        for i in 0..jbi_len_m1 {
            let j: u32 = *jbi.get(&i).unwrap();
            if j > b_max {
                b_max = j
            };
        }
        // find the first index that has the max joltage
        // and then find the largest joltage battery
        // after that.
        //
        let mut idx_1st: u32 = u32::MAX;
        let mut b_max2: u32 = 0;
        let mut idx_2nd: u32 = u32::MAX;
        for i in 0..jbi_len {
            let j: u32 = *jbi.get(&i).unwrap();
            if (idx_1st == u32::MAX) && (j == b_max) {
                idx_1st = i;
            } else if (idx_1st != u32::MAX) && (j > b_max2) {
                b_max2 = j;
                idx_2nd = i;
            }
        }
        BatteryBank {
            joltage_by_idx: jbi,
            idx_1st_largest: idx_1st,
            idx_2nd_largest: idx_2nd,
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
    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        let battery_bank = BatteryBank::new(line.trim());
        let max_joltage = battery_bank.max_joltage().unwrap();
        joltages.push(max_joltage);
        // println!("For '{}' max is {}", line.trim(), max_joltage);
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
fn check_ascending_descending() {
    let ss = String::from("12345678987654321");
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
fn check_last_biggest() {
    let ss = String::from("876543219");
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
