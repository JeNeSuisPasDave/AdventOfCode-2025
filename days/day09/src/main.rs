use ::std::cmp::Ordering;
use ::std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Id, Parser};
use regex::Regex;

/// Given input file containing the coordinates of red tiles,
/// find the largest area bounded by red tiles as opposite corners.
///
#[derive(Parser, Debug)]
struct Cli {
    /// The path to the file containing red tile coordinates
    path: PathBuf,
}

#[derive(Debug)]
struct Point {
    x: u64, // column
    y: u64, // row
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x: x, y: y }
    }

    fn area_with(&self, other: &Point) -> u64 {
        if (self.x == other.x) || (self.y == other.y) {
            0
        } else if self.x < other.x {
            if self.y < other.y {
                let dx = other.x - self.x;
                let dy = other.y - self.y;
                (dx + 1) * (dy + 1)
            } else {
                let dx = other.x - self.x;
                let dy = self.y - other.y;
                (dx + 1) * (dy + 1)
            }
        } else {
            if self.y < other.y {
                let dx = self.x - other.x;
                let dy = other.y - self.y;
                (dx + 1) * (dy + 1)
            } else {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                (dx + 1) * (dy + 1)
            }
        }
    }
}

fn find_max_area(
    max_area: &mut u64,
    points: &Vec<Point>,
    rng: Range<usize>,
) {
    let id_a: usize = rng.start;
    let end: usize = rng.end;
    if 1 >= (end - id_a) {
        return;
    }
    let start = id_a + 1;
    find_max_area(max_area, points, start..end);
    let point_a = points.get(id_a).unwrap();
    for id_b in start..end {
        let point_b = points.get(id_b).unwrap();
        let area = point_a.area_with(point_b);
        if area > *max_area {
            *max_area = area
        }
    }
}

fn file_to_points(f: File) -> Vec<Point> {
    let rdr = BufReader::new(f);
    let lines = rdr.lines();
    let mut points: Vec<Point> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*$").unwrap();
    let mut line_num: usize = 0;
    for line in lines {
        line_num += 1;
        let line = line.unwrap();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        if !re_coord.is_match(&line) {
            println!(
                "*** FAILED *** to match line {}: '{}'",
                line_num, line
            );
            continue;
        }
        let coords = re_coord.captures(&line).unwrap();
        let xs = coords.get(1).unwrap().as_str();
        let x = xs.parse::<u64>().unwrap();
        let ys = coords.get(2).unwrap().as_str();
        let y = ys.parse::<u64>().unwrap();
        let p = Point::new(x, y);
        points.push(p);
    }
    points
}

fn string_to_points(raw_input: String) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*$").unwrap();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    let mut line_num: usize = 0;
    for line in lines {
        line_num += 1;
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        if !re_coord.is_match(&line) {
            println!(
                "*** FAILED *** to match line {}: '{}'",
                line_num, line
            );
            continue;
        }
        let coords = re_coord.captures(&line).unwrap();
        let xs = coords.get(1).unwrap().as_str();
        let x = xs.parse::<u64>().unwrap();
        let ys = coords.get(2).unwrap().as_str();
        let y = ys.parse::<u64>().unwrap();
        let p = Point::new(x, y);
        points.push(p);
    }
    points
}

// Binary crate entry point
//
fn main() -> Result<()> {
    let args = Cli::parse();
    let mut upto: usize = 10;
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let points = file_to_points(f);

    let mut max_area: u64 = 0;
    let len = points.len();
    find_max_area(&mut max_area, &points, 0..len);

    println!("Max area: {}", max_area);

    Ok(())
}

#[test]
fn t_on_same_row() {
    let a = Point::new(7, 11);
    let b = Point::new(2, 11);
    let area = a.area_with(&b);
    assert_eq!(0, area);
}

#[test]
fn t_on_same_column() {
    let a = Point::new(7, 11);
    let b = Point::new(7, 2);
    let area = a.area_with(&b);
    assert_eq!(0, area);
}

#[test]
fn t_ul_br() {
    let a = Point::new(1, 1);
    let b = Point::new(5, 4);
    let area = a.area_with(&b);
    assert_eq!(20, area);
}

#[test]
fn t_ur_bl() {
    let a = Point::new(5, 1);
    let b = Point::new(1, 4);
    let area = a.area_with(&b);
    assert_eq!(20, area);
}

#[test]
fn t_bl_ur() {
    let a = Point::new(1, 4);
    let b = Point::new(5, 1);
    let area = a.area_with(&b);
    assert_eq!(20, area);
}

#[test]
fn t_br_ul() {
    let a = Point::new(5, 4);
    let b = Point::new(1, 1);
    let area = a.area_with(&b);
    assert_eq!(20, area);
}

#[test]
fn t_given_example_part1() {
    let raw_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
    .to_string();
    let points = string_to_points(raw_input);

    let mut max_area: u64 = 0;
    let len = points.len();
    find_max_area(&mut max_area, &points, 0..len);

    assert_eq!(50, max_area);
}
