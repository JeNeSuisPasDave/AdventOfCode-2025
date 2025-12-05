use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Bound::{Included, Unbounded};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the ingredient database,
/// identify and count the fresh ingredients. Output the
/// number of fresh ingredients.
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

// models a range of ingredient IDs
//
#[derive(Debug)]
struct IngredientRange {
    start: u64,
    end: u64,
}

// functions associated with struct IngredientRange
//
impl IngredientRange {
    // constructor
    //
    fn new(start: u64, end: u64) -> Self {
        if start > end {
            panic!(
                "start of range must by <= end, but found {} > {}",
                start, end
            );
        }
        IngredientRange {
            start: start,
            end: end,
        }
    }

    // Returns true if id is within the range; otherwise false
    //
    fn contains(&self, id: u64) -> bool {
        if id < self.start {
            false
        } else if id > self.end {
            false
        } else {
            true
        }
    }
}

// models an ingredient database
//
struct IngredientDB {
    // A list of ingredient ranges in the order added
    //
    ranges: Vec<IngredientRange>,
    // A list of ingredient ranges sorted by starting ID
    //
    fresh_ranges_by_start: BTreeMap<u64, usize>,
    // A list of ingredient ranges sorted by ending ID
    //
    fresh_ranges_by_end: BTreeMap<u64, usize>,
}

// functions associated with IngredientDB
//
impl IngredientDB {
    // constructor
    //
    fn new() -> Self {
        let list: Vec<IngredientRange> = Vec::new();
        let dict_by_start: BTreeMap<u64, usize> = BTreeMap::new();
        let dict_by_end: BTreeMap<u64, usize> = BTreeMap::new();
        IngredientDB {
            ranges: list,
            fresh_ranges_by_start: dict_by_start,
            fresh_ranges_by_end: dict_by_end,
        }
    }

    // add a new fresh ingredient range
    //
    fn add_range(&mut self, start: u64, end: u64) {
        let ir = IngredientRange::new(start, end);
        self.ranges.push(ir);
        let idx: usize = self.ranges.len().try_into().unwrap();
        let range_idx: usize = idx - 1;
        self.fresh_ranges_by_start.insert(start, range_idx);
        self.fresh_ranges_by_end.insert(end, range_idx);
    }

    // check whether the ingredient is known to be fresh
    //
    fn is_fresh(&self, id: u64, brute_force: bool) -> bool {
        // println!("Checking freshness of {}", id);
        let mut result: bool = false;
        if brute_force {
            for thing in self.ranges.iter() {
                if thing.contains(id) {
                    result = true;
                    break;
                }
            }
        } else {
            // iterator for all ranges that start at or below the id
            //
            let up_bounds = (Unbounded, Included(id));
            let up_from = self.fresh_ranges_by_start.range(up_bounds);
            //
            // iterator for all ranges that end at or above the id
            //
            let down_bounds = (Included(id), Unbounded);
            let down_to = self.fresh_ranges_by_start.range(down_bounds);
            //
            //
            for thing in up_from {
                let (_, range_idx): (&u64, &usize) = thing;
                // println!("Checking up_from at idx {0}", range_idx);
                let val: &IngredientRange =
                    self.ranges.get(*range_idx).unwrap();
                if val.contains(id) {
                    result = true;
                    break;
                }
            }
            if !result {
                for thing in down_to {
                    let (_, range_idx): (&u64, &usize) = thing;
                    // println!("Checking down_to at idx {0}", range_idx);
                    let val: &IngredientRange =
                        self.ranges.get(*range_idx).unwrap();
                    if val.contains(id) {
                        result = true;
                        break;
                    }
                }
            }
        }
        result
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

    // populate the DB with fresh ingredient ranges. To do
    // that, read in the ranges until a blank line is encountered
    //
    let mut fresh_ingredient_count: u64 = 0;
    let mut total_range_count: u64 = 0;
    let mut total_ingredient_count: u64 = 0;
    let mut spoiled_ingredient_count: u64 = 0;
    let mut db = IngredientDB::new();
    let mut process_ids: bool = false;
    for line in lines {
        let line = line.with_context(|| {
            format!("Problem reading from `{}`", path.display())
        })?;
        let line = line.trim();
        if 0 == line.len() {
            process_ids = true;
            continue;
        }
        if !process_ids {
            total_range_count += 1;
            // process ranges
            //
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts.get(0).unwrap().parse().unwrap();
            let end: u64 = parts.get(1).unwrap().parse().unwrap();
            db.add_range(start, end);
        } else {
            total_ingredient_count += 1;
            let id: u64 = line.parse().unwrap();
            if db.is_fresh(id, true) {
                fresh_ingredient_count += 1;
                // println!("FRESH: {}", id);
            } else {
                spoiled_ingredient_count += 1;
                // println!("spoiled: {}", id);
            }
        }
    }

    // Display the total number of fresh ingredients
    //
    println!(
        "The count of fresh ingredients is {}",
        fresh_ingredient_count
    );
    println!(
        "The count of spoiled ingredients is {}",
        spoiled_ingredient_count
    );
    println!(
        "The count of total ingredients is {}",
        total_ingredient_count
    );
    println!("The count of ranges is {}", total_range_count);
    Ok(())
}
