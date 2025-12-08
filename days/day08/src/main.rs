use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

/// Given input file containing the problem set,
/// establish the circuits and return the product
/// of the size (junction box count) of the three
/// argest circuits.
///
#[derive(Parser)]
struct Cli {
    /// the maximum number of circuits to assemble
    upto: usize,
    /// the number of the largest circuits from which
    /// to produce the product of their sizes
    productoflargest: usize,
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x: x, y: y, z: z }
    }

    fn distance_from(&self, other: &Point) -> f64 {
        let mut p1 = [0.0_f64; 3];
        let mut p2 = [0.0_f64; 3];
        p1[0] = f64::from(self.x);
        p1[1] = f64::from(self.y);
        p1[2] = f64::from(self.z);
        p2[0] = f64::from(other.x);
        p2[1] = f64::from(other.y);
        p2[2] = f64::from(other.z);
        let sum_of_squares = (p1[0] - p2[0]).abs().powi(2)
            * (p1[1] - p2[1]).abs().powi(2)
            * (p1[2] - p2[2]).abs().powi(2);
        sum_of_squares.sqrt()
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

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
    }

    // Display the grand total of problem answers
    //
    let path_count: usize = 0;
    println!("The path count is {}", path_count);
    Ok(())
}

// test with example input
//
#[test]
fn given_example_part1() {
    // the maximum number of circuits to assemble
    let upto: usize = 10;
    // the number of the largest circuits from which
    // to produce the product of their sizes
    let productoflargest: usize = 3;
    let expected_product: usize = 40;
    let raw_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        .to_string();
    let mut points: Vec<Point> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*$")
            .unwrap();
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
        let x = xs.parse::<i32>().unwrap();
        let ys = coords.get(2).unwrap().as_str();
        let y = ys.parse::<i32>().unwrap();
        let zs = coords.get(3).unwrap().as_str();
        let z = zs.parse::<i32>().unwrap();
        let p: Point = Point::new(x, y, z);
        println!("{:#?}", p);
        points.push(p);
    }
    println!("Read in {0} points", points.len());
    let actual_product: usize = 0;
    assert_eq!(expected_product, actual_product);
}
