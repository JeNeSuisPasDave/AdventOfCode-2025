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
    start: u32,
    end: u32,
}

static IDRANGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*([0-9]+)-([0-9]+)\s*,?\s*$").unwrap()
});

impl IdRange {
    fn new(start: u32, end: u32) -> Self {
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
        let start: u32 = sstr.parse::<u32>().unwrap();
        let end: u32 = estr.parse::<u32>().unwrap();
        Some(IdRange::new(start, end))
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

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
            println!("Range: {}-{}", idr.start, idr.end);
        }
    }
    if s.len() > 0 {
        let ss = s.iter().collect::<String>();
        s.clear();
        let idr = IdRange::new_from_str(&ss);
        if idr.is_some() {
            let idr = idr.unwrap();
            println!("Range: {}-{}", idr.start, idr.end);
        }
    }

    Ok(())
}
