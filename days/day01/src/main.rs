use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

/// Given input file containing the safe dial operations,
/// determine the password.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing dial operations
    path: PathBuf,
}

#[derive(Debug)]
struct Dial {
    zero_count: u32,
    position: u32,
    len: u32,
}

impl Dial {
    fn new(len: u32) -> Self {
        Self {
            zero_count: 0,
            position: 50,
            len: len,
        }
    }

    fn new_default() -> Self {
        Self::new(100)
    }

    fn left(&mut self, clicks: u32) {
        let d = clicks % self.len;
        if d <= self.position {
            self.position -= d;
        } else {
            self.position = self.len + self.position - d;
        }
        if self.position == 0 {
            self.zero_count += 1;
        }
    }

    fn right(&mut self, clicks: u32) {
        let d = clicks % self.len;
        if d <= ((self.len - 1) - self.position) {
            self.position += d;
        } else {
            self.position = self.position + d - self.len;
        }
        if self.position == 0 {
            self.zero_count += 1;
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let rdr = BufReader::new(f);
    let lines = rdr.lines();

    let mut dial = Dial::new_default();
    let re_inst = Regex::new(r"([LR])([0-9]+)").unwrap();

    // dial.left(1);
    // dial.right(1);

    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        if !re_inst.is_match(&line) {
            println!("*** FAILED *** to match {}", line);
            continue;
        }
        let caps = re_inst.captures(&line).unwrap();
        let dir: &str = caps.get(1).unwrap().as_str();
        let dist: &str = caps.get(2).unwrap().as_str();
        let dist: u32 = dist.parse::<u32>().unwrap();
        if "L".eq(dir) {
            dial.left(dist);
        } else if "R".eq(dir) {
            dial.right(dist);
        }
        println!(
            "Turn {} {} clicks; zero count is {}.",
            dir, dist, dial.zero_count
        );
    }
    Ok(())
}

// left tests
//
#[test]
fn check_left_before_zero() {
    let mut dial = Dial::new_default();
    dial.left(49);
    assert_eq!(dial.position, 1);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_left_to_zero() {
    let mut dial = Dial::new_default();
    dial.left(50);
    assert_eq!(dial.position, 0);
    assert_eq!(dial.zero_count, 1);
}

#[test]
fn check_left_beyond_zero() {
    let mut dial = Dial::new_default();
    dial.left(55);
    assert_eq!(dial.position, 95);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_left_before_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.left(349);
    assert_eq!(dial.position, 1);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_left_to_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.left(250);
    assert_eq!(dial.position, 0);
    assert_eq!(dial.zero_count, 1);
}

#[test]
fn check_left_beyond_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.left(155);
    assert_eq!(dial.position, 95);
    assert_eq!(dial.zero_count, 0);
}

// right tests
//
#[test]
fn check_right_before_zero() {
    let mut dial = Dial::new_default();
    dial.right(49);
    assert_eq!(dial.position, 99);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_right_to_zero() {
    let mut dial = Dial::new_default();
    dial.right(50);
    assert_eq!(dial.position, 0);
    assert_eq!(dial.zero_count, 1);
}

#[test]
fn check_right_beyond_zero() {
    let mut dial = Dial::new_default();
    dial.right(55);
    assert_eq!(dial.position, 5);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_right_before_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.right(349);
    assert_eq!(dial.position, 99);
    assert_eq!(dial.zero_count, 0);
}

#[test]
fn check_right_to_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.right(250);
    assert_eq!(dial.position, 0);
    assert_eq!(dial.zero_count, 1);
}

#[test]
fn check_right_beyond_zero_wrapped() {
    let mut dial = Dial::new_default();
    dial.right(155);
    assert_eq!(dial.position, 5);
    assert_eq!(dial.zero_count, 0);
}
