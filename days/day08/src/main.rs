use ::std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;
use std::usize;

use anyhow::{Context, Result};
use clap::{Id, Parser};
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
enum CircuitMergeKind {
    FirstToFirst,
    FirstToLast,
    LastToFirst,
    LastToLast,
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
        let sum_of_squares: f64 = (p1[0] - p2[0]).abs().powi(2)
            + (p1[1] - p2[1]).abs().powi(2)
            + (p1[2] - p2[2]).abs().powi(2);
        sum_of_squares.sqrt()
    }
}

#[derive(Debug)]
struct Circuit {
    boxes: Vec<Point>,
    first_idx: usize,
    last_idx: usize,
}

impl Circuit {
    // constructor
    //
    fn new(point: Point) -> Self {
        let mut boxes: Vec<Point> = Vec::new();
        boxes.push(point);
        Circuit {
            boxes: boxes,
            first_idx: 0,
            last_idx: 0,
        }
    }

    // returns the number of junction boxes in the circuit
    //
    fn box_count(&self) -> usize {
        self.boxes.len()
    }

    // returns the location of the first box in the circuit
    //
    fn first_location(&self) -> &Point {
        self.boxes.get(self.first_idx).unwrap()
    }

    // returns the location of the last box in the circuit
    //
    fn last_location(&self) -> &Point {
        self.boxes.get(self.last_idx).unwrap()
    }

    fn merge_with(
        &mut self,
        other: &mut Self,
        guide: CircuitMergeKind,
    ) {
        match guide {
            CircuitMergeKind::FirstToFirst => {
                while 0 < other.boxes.len() {
                    let p = other.boxes.remove(0);
                    self.boxes.insert(0, p);
                }
            }
            CircuitMergeKind::FirstToLast => {
                while 0 < other.boxes.len() {
                    let p = other.boxes.pop().unwrap();
                    self.boxes.insert(0, p);
                }
            }
            CircuitMergeKind::LastToFirst => {
                while 0 < other.boxes.len() {
                    let p = other.boxes.remove(0);
                    self.boxes.push(p);
                }
            }
            CircuitMergeKind::LastToLast => {
                while 0 < other.boxes.len() {
                    let p = other.boxes.pop().unwrap();
                    self.boxes.push(p);
                }
            }
        }
        self.first_idx = 0;
        self.last_idx = self.boxes.len() - 1;
    }
}

// find the closest two circuits
//
// Returns (index of circuit being compared,
// index of closest circuit, circuit merge instructions)
//
fn find_closest_circuits(
    circuits: &Vec<Circuit>,
) -> (usize, usize, CircuitMergeKind) {
    let mut closest_distance = f64::MAX;
    let mut closest_idx_a = usize::MAX;
    let mut closest_idx_b = usize::MAX;
    let mut closest_mrginst = CircuitMergeKind::FirstToFirst;
    let len = circuits.len();
    find_closest_circuit(
        circuits,
        0..len,
        &mut closest_distance,
        &mut closest_idx_a,
        &mut closest_idx_b,
        &mut closest_mrginst,
    );
    (closest_idx_a, closest_idx_b, closest_mrginst)
}

fn find_closest_circuit(
    circuits: &Vec<Circuit>,
    rng: Range<usize>,
    closest_distance: &mut f64,
    closest_idx_a: &mut usize,
    closest_idx_b: &mut usize,
    closest_mrginst: &mut CircuitMergeKind,
) {
    let idx: usize = rng.start;
    let end: usize = rng.end;
    if 1 >= (end - idx) {
        return;
    }
    let start = idx + 1;
    find_closest_circuit(
        circuits,
        start..end,
        closest_distance,
        closest_idx_a,
        closest_idx_b,
        closest_mrginst,
    );
    for other_idx in start..end {
        let a: &Circuit = &(circuits[idx]);
        let b: &Circuit = &(circuits[other_idx]);
        // if both circuits have multiple boxes, don't
        // consider because we are only connecting isolated
        // junction boxes
        //
        if (1 < a.box_count()) && (1 < b.box_count()) {
            continue;
        }
        // otherwise, at least one circuit is isolated, so
        // consider the distance to it
        //
        let mut d =
            a.first_location().distance_from(b.first_location());
        if d < *closest_distance {
            *closest_distance = d;
            *closest_idx_a = idx;
            *closest_idx_b = other_idx;
            *closest_mrginst = CircuitMergeKind::FirstToFirst;
            println!(
                "close: {}, {}, {}",
                closest_idx_a, closest_idx_b, closest_distance
            );
        }
        if 0 < a.box_count() {
            d = a.last_location().distance_from(b.first_location());
            if d < *closest_distance {
                *closest_distance = d;
                *closest_idx_a = idx;
                *closest_idx_b = other_idx;
                *closest_mrginst = CircuitMergeKind::LastToFirst;
                println!(
                    "close: {}, {}, {}",
                    closest_idx_a, closest_idx_b, closest_distance
                );
            }
        }
        if 0 < b.box_count() {
            d = a.first_location().distance_from(b.last_location());
            if d < *closest_distance {
                *closest_distance = d;
                *closest_idx_a = idx;
                *closest_idx_b = other_idx;
                *closest_mrginst = CircuitMergeKind::FirstToLast;
                println!(
                    "close: {}, {}, {}",
                    closest_idx_a, closest_idx_b, closest_distance
                );
            }
            if 0 < a.box_count() {
                d = a.last_location().distance_from(b.last_location());
                if d < *closest_distance {
                    *closest_distance = d;
                    *closest_idx_a = idx;
                    *closest_idx_b = other_idx;
                    *closest_mrginst = CircuitMergeKind::LastToLast;
                    println!(
                        "close: {}, {}, {}",
                        closest_idx_a, closest_idx_b, closest_distance
                    );
                }
            }
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

#[test]
fn check_distance_1() {
    let a = Point::new(162, 187, 812);
    let b = Point::new(425, 690, 689);
    let dist = a.distance_from(&b);
    assert!((dist - (580.781370_f64)).abs() < 1e-6);
}

#[test]
fn check_distance_2() {
    let a = Point::new(739, 650, 466);
    let b = Point::new(346, 949, 466);
    let dist = a.distance_from(&b);
    assert!((dist - (493.811705)).abs() < 1e-6);
}

// test with example input, one pass
//
#[test]
fn given_example_part1_full() {
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
    let mut circuits: Vec<Circuit> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*$")
            .unwrap();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    let mut line_num: usize = 0;
    let mut idx: usize = 0;
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
        let circuit: Circuit = Circuit::new(p);
        circuits.push(circuit);
        idx += 1;
    }
    println!("Read in {} points", circuits.len());

    // find the n closest circuits
    //
    for _i in 0..upto {
        let (idx_a, idx_b, mrginst) = find_closest_circuits(&circuits);
        let mut circuit_b: Circuit = circuits.remove(idx_b);
        let circuit_a: &mut Circuit = circuits.get_mut(idx_a).unwrap();
        println!("merging {} with {}", idx_a, idx_b);
        circuit_a.merge_with(&mut circuit_b, mrginst);
        println!(
            "merged ... count: {}; circuits count: {}",
            circuit_a.box_count(),
            circuits.len()
        );
    }

    // sort circuits by size and id
    //
    let mut largest_circuits: Vec<usize> =
        (0..circuits.len()).collect();
    println!("largest_circuits length is {}", largest_circuits.len());
    largest_circuits.sort_by(|a, b| {
        println!("Compare {} with {}", a, b);
        let c_a: &Circuit = circuits.get(*a).unwrap();
        let c_b: &Circuit = circuits.get(*b).unwrap();
        if c_a.box_count() > c_b.box_count() {
            Ordering::Less
        } else if c_a.box_count() < c_b.box_count() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut actual_product: usize = 1;

    for i in largest_circuits.iter() {
        println!(
            "largest {} has {} boxes",
            i,
            circuits[*i].box_count()
        );
    }
    for i in 0..productoflargest {
        actual_product *=
            circuits.get(largest_circuits[i]).unwrap().box_count();
    }
    assert_eq!(expected_product, actual_product);
}

// test with example input, one pass
//
#[test]
fn given_example_part1_pass1() {
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
    let mut circuits: Vec<Circuit> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*$")
            .unwrap();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    let mut line_num: usize = 0;
    let mut idx: usize = 0;
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
        let circuit: Circuit = Circuit::new(p);
        circuits.push(circuit);
        idx += 1;
    }
    println!("Read in {} points", circuits.len());

    // find the closest circuits
    //
    let (idx_a, idx_b, mrginst) = find_closest_circuits(&circuits);
    assert_eq!(0, idx_a);
    assert_eq!(19, idx_b);
    match mrginst {
        CircuitMergeKind::FirstToFirst => {}
        _ => {
            panic!("unexpected enum value {:#?}", mrginst);
        }
    }
}
