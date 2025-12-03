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
    joltage_by_idx: BTreeMap<u32, u64>,
}

// methods and associated methods for the BatteryBank struct
//
impl BatteryBank {
    fn new(spec: &str) -> Self {
        // load an indexed map with the joltage values
        //
        let mut jbi: BTreeMap<u32, u64> = BTreeMap::new();
        for (ii, c) in spec.chars().enumerate() {
            let radix = 10;
            if !c.is_digit(radix) {
                break;
            }
            let i = ii.try_into().unwrap();
            let jj = c.to_digit(radix).unwrap();
            let j = jj.try_into().unwrap();
            jbi.insert(i, j);
        }
        BatteryBank {
            joltage_by_idx: jbi,
        }
    }

    fn find_first_largest(
        &self,
        idx_from: u32,
        idx_to: u32,
    ) -> Option<u32> {
        let jbi = &self.joltage_by_idx;
        let mut idx: u32 = u32::MAX;
        let mut j_max: u64 = 0;
        for i in idx_from..idx_to {
            let j: u64 = *jbi.get(&i).unwrap();
            if j > j_max {
                j_max = j;
                idx = i;
            }
        }
        if idx == u32::MAX { None } else { Some(idx) }
    }

    fn max_joltage(&self, battery_count: u32) -> Option<u64> {
        let jbi = &self.joltage_by_idx;
        let jbi_len: u32 = jbi.len().try_into().unwrap();
        // if there are fewer batteries in the bank than requested
        // by battery_count, then return None.
        //
        if battery_count > jbi_len {
            return None;
        }
        //
        // otherwise, loop through the range of batteries
        // that can be considered for each unidentified
        // battery, identifying the first battery with the
        // largest joltage.
        //
        let mut batteries: Vec<u32> = Vec::new();
        let mut remaining_battery_count: u32 = battery_count;
        let mut idx_start: u32 = 0;
        let mut idx_up_to: u32 = jbi_len - remaining_battery_count + 1;
        for _battery in 0..battery_count {
            match self.find_first_largest(idx_start, idx_up_to) {
                None => return None,
                Some(idx) => {
                    batteries.push(idx);
                    remaining_battery_count -= 1;
                    idx_start = idx + 1;
                    idx_up_to = jbi_len - remaining_battery_count + 1;
                }
            }
        }
        //
        // Now construct the joltage of the selected batteries
        //
        let mut selected_joltage: u64 = 0;
        for idx in batteries {
            selected_joltage =
                selected_joltage * 10 + *jbi.get(&idx).unwrap();
        }
        Some(selected_joltage)
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
    let mut joltages: Vec<u64> = Vec::new();
    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        let battery_bank = BatteryBank::new(line.trim());
        let max_joltage = battery_bank.max_joltage(12).unwrap();
        joltages.push(max_joltage);
        // println!("For '{}' max is {}", line.trim(), max_joltage);
    }

    // add up the max joltage for each bank
    //
    let mut joltage_accum: u64 = 0;
    for joltage in joltages {
        joltage_accum += joltage;
    }
    println!("The total joltage is {}.", joltage_accum);
    Ok(())
}

// BatteryBank tests with 2 batteries
//

#[test]
fn check_all_ones() {
    let ss = String::from("1111111111111111");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 11;
    match bb.max_joltage(2) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_ascending() {
    let ss = String::from("1234567899999999");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 99;
    match bb.max_joltage(2) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_ascending_descending() {
    let ss = String::from("1234567898765432");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 98;
    match bb.max_joltage(2) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_descending() {
    let ss = String::from("9876543219876543");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 99;
    match bb.max_joltage(2) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_last_biggest() {
    let ss = String::from("8181568765432119");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 89;
    match bb.max_joltage(2) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

// BatteryBank tests with 12 batteries
//

#[test]
fn check_all_ones_12() {
    let ss = String::from("1111111111111111");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 111111111111;
    match bb.max_joltage(12) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_ascending_12() {
    let ss = String::from("1234567899999999");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 567899999999;
    match bb.max_joltage(12) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_ascending_descending_12() {
    let ss = String::from("1234567898765432");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 567898765432;
    match bb.max_joltage(12) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_descending_12() {
    let ss = String::from("9876543219876543");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 987659876543;
    match bb.max_joltage(12) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}

#[test]
fn check_last_biggest_12() {
    let ss = String::from("8181568765432119");
    let s: &str = ss.as_str();
    let bb = BatteryBank::new(s);
    let expected: u64 = 888765432119;
    match bb.max_joltage(12) {
        None => {
            assert!(false, "FAILED to find max joltage")
        }
        Some(actual) => assert_eq!(expected, actual),
    }
}
