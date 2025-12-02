use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use utf8_chars::BufReadCharsExt;

/// Given input file containing the comma-separated list of product ID
/// ranges, determine the sum of all invalid product IDs.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing product ID ranges
    path: PathBuf,
}

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

static IDRANGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*([0-9]+)-([0-9]+)\s*,?\s*$").unwrap()
});

impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        Self {
            start: start,
            end: end,
        }
    }

    fn new_from_str(id_range: &str) -> Option<Self> {
        if !IDRANGE_RE.is_match(id_range) {
            println!("*** FAILED *** to match range '{}'", id_range);
            return None;
        }
        let caps = IDRANGE_RE.captures(&id_range).unwrap();
        let sstr: &str = caps.get(1).unwrap().as_str();
        let estr: &str = caps.get(2).unwrap().as_str();
        let start: u64 = sstr.parse::<u64>().unwrap();
        let end: u64 = estr.parse::<u64>().unwrap();
        Some(IdRange::new(start, end))
    }

    fn invalid_ids(self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        let start_s = self.start.to_string();
        let end_s = self.end.to_string();

        // if odd number of digits and both start and end
        // have the same magnitude, then there are no
        // invalid IDs in the range
        //
        if (start_s.len() == end_s.len())
            && (start_s.len() % 2 == 1)
            && (end_s.len() % 2 == 1)
        {
            return result;
        }

        // 'num' will be the variable to hold the ID to be
        // scanned.
        //
        let mut num: u64 = self.start;

        // if 'num' has an odd number of digits, jump to the
        // next power of 10
        //
        let s = num.to_string();
        if s.len() % 2 == 1 {
            // println!("wat");
            let exp: u32 = s.len() as u32;
            num = u64::pow(10, exp);
        }

        let mag: u32 = (num.to_string().len() as u32) - 1; // power of 10
        let half_mag: u32 = mag / 2;
        // println!("num: {}; mag: {}; half_mag: {}", num, mag, half_mag);
        let mut inc: u64 = u64::pow(10, half_mag + 1);
        let mut half_num: u64 = num / inc;
        let mut half_num_max: u64 = u64::pow(10, half_mag + 1);
        loop {
            num = (half_num * inc) + half_num;
            if num > self.end {
                break;
            }
            if num >= self.start {
                result.push(num);
            }
            half_num += 1;
            // if we've jumped up to the next power of 10, then
            // that will be an odd pairing, so we need to jump
            // yet another power of 10 and then keep looking
            //
            if half_num >= half_num_max {
                half_num = half_num_max * 10;
                inc *= 100;
                half_num_max *= 100;
            }
        }
        return result;
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

    let mut invalid_id_accum: u64 = 0;
    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let mut rdr = BufReader::new(f);
    let mut s = Vec::new();
    for c in rdr.chars().map(|x| x.unwrap()) {
        s.push(c);
        if c == ',' {
            let ss = s.iter().collect::<String>();
            s.clear();
            let idr = IdRange::new_from_str(&ss);
            if idr.is_none() {
                continue;
            }
            let idr = idr.unwrap();
            println!(
                "Range: {}-{}; accum: {}",
                idr.start, idr.end, invalid_id_accum
            );
            for invalid_id in idr.invalid_ids() {
                // println!("Invalid ID: {}", invalid_id);
                invalid_id_accum += invalid_id;
            }
        }
    }
    if s.len() > 0 {
        let ss = s.iter().collect::<String>();
        s.clear();
        let idr = IdRange::new_from_str(&ss);
        if idr.is_some() {
            let idr = idr.unwrap();
            // println!("Range: {}-{}", idr.start, idr.end);
            for invalid_id in idr.invalid_ids() {
                // println!("Invalid ID: {}", invalid_id);
                invalid_id_accum += invalid_id;
            }
        }
    }
    println!("Sum of invalid_ids: {}", invalid_id_accum);

    Ok(())
}
