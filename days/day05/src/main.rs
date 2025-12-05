use std::fs::File;
use std::io::{BufRead, BufReader};
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

    // copy constructor
    //
    fn copy(&self) -> Self {
        IngredientRange {
            start: self.start,
            end: self.end,
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

    fn overlaps_range(&self, other: &IngredientRange) -> bool {
        if (other.start >= self.start)
            && (other.start <= self.end)
            && (other.end > self.end)
        {
            true
        } else if (other.end >= self.start)
            && (other.end <= self.end)
            && (other.start < self.start)
        {
            true
        } else {
            false
        }
    }

    fn contains_range(&self, other: &IngredientRange) -> bool {
        if (other.start >= self.start)
            && (other.start <= self.end)
            && (other.end >= self.start)
            && (other.end <= self.end)
        {
            true
        } else {
            false
        }
    }

    fn contained_by_range(&self, other: &IngredientRange) -> bool {
        if (self.start >= other.start)
            && (self.start <= other.end)
            && (self.end >= other.start)
            && (self.end <= other.end)
        {
            true
        } else {
            false
        }
    }

    fn merge_with(&mut self, other: &IngredientRange) {
        let new_start = u64::min(self.start, other.start);
        let new_end = u64::max(self.end, other.end);
        self.start = new_start;
        self.end = new_end;
    }
}

// models an ingredient database
//
struct IngredientDB {
    // A list of ingredient ranges in the order added
    //
    original_ranges: Vec<IngredientRange>,
    merged_ranges: Vec<IngredientRange>,
}

// functions associated with IngredientDB
//
impl IngredientDB {
    // constructor
    //
    fn new() -> Self {
        let list1: Vec<IngredientRange> = Vec::new();
        let list2: Vec<IngredientRange> = Vec::new();
        IngredientDB {
            original_ranges: list1,
            merged_ranges: list2,
        }
    }

    // add a new fresh ingredient range
    //
    fn add_range(&mut self, start: u64, end: u64) {
        let ir = IngredientRange::new(start, end);
        self.original_ranges.push(ir);
        let ir = IngredientRange::new(start, end);
        self.update_merged_ranges(&ir);
    }

    // check whether the ingredient is known to be fresh
    //
    fn is_fresh(&self, id: u64) -> bool {
        // println!("Checking freshness of {}", id);
        let mut result: bool = false;
        for thing in self.merged_ranges.iter() {
            if thing.contains(id) {
                result = true;
                break;
            }
        }
        result
    }

    fn update_merged_ranges(&mut self, ir: &IngredientRange) {
        let mut ir_was_merged: bool = false;
        let mut unchanged_ranges: Vec<IngredientRange> = Vec::new();
        let mut new_range: IngredientRange = ir.copy();
        for thing in self.merged_ranges.iter() {
            if thing.contains_range(&new_range)
                || thing.contained_by_range(&new_range)
                || thing.overlaps_range(&new_range)
            {
                ir_was_merged = true;
                let mut merged_range = thing.copy();
                merged_range.merge_with(&new_range);
                new_range = merged_range.copy();
            } else {
                unchanged_ranges.push(thing.copy());
            }
        }
        //
        // update the merged_ranges collection
        //
        if !ir_was_merged {
            // new range was not merged, so add it to the list
            //
            self.merged_ranges.push(ir.copy());
        } else {
            // one or more ranges were merged, so recreated
            // the merged_range collection by assemblying the
            // unchanged ranges and the new merged range
            //
            self.merged_ranges = Vec::new();
            for ur in unchanged_ranges {
                self.merged_ranges.push(ur);
            }
            self.merged_ranges.push(new_range);
        }
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
            if db.is_fresh(id) {
                fresh_ingredient_count += 1;
                // println!("FRESH: {}", id);
            } else {
                spoiled_ingredient_count += 1;
                // println!("spoiled: {}", id);
            }
        }
    }

    // Calculate total possible fresh ingredients
    //
    let mut total_possible_fresh_ingredients: u64 = 0;
    for ir in db.merged_ranges.iter() {
        let size_of_range: u64 = (ir.end + 1) - ir.start;
        total_possible_fresh_ingredients += size_of_range;
    }
    let total_merged_ranges: u64 =
        db.merged_ranges.len().try_into().unwrap();

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
    println!("The count of merged ranges is {}", total_merged_ranges);
    println!(
        "The total possible fresh ingredients is {}",
        total_possible_fresh_ingredients
    );
    Ok(())
}
