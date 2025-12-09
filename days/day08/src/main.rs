use ::std::cmp::Ordering;
use ::std::collections::{BTreeMap, BTreeSet};
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
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x: x, y: y, z: z }
    }

    fn distance_from(&self, other: &Point) -> u64 {
        let dx: u64 = (self.x - other.x).abs().try_into().unwrap();
        let dy: u64 = (self.y - other.y).abs().try_into().unwrap();
        let dz: u64 = (self.z - other.z).abs().try_into().unwrap();
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
struct JunctionBox {
    location: Point,
    id: usize,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64, id: usize) -> Self {
        let p: Point = Point::new(x, y, z);
        JunctionBox {
            location: p,
            id: id,
        }
    }

    fn distance_from(&self, other: &Self) -> u64 {
        self.location.distance_from(&other.location)
    }

    fn describe_coords(&self) -> String {
        format!(
            "({},{},{})",
            self.location.x, self.location.y, self.location.z
        )
    }
}

#[derive(Debug)]
struct Circuit<'a> {
    // contains references to junction boxes that make
    // up this circuit
    //
    junction_boxes: BTreeMap<usize, &'a JunctionBox>,
}

impl<'a> Circuit<'a> {
    // constructor
    //
    fn new(junction_box: &'a JunctionBox) -> Self {
        let mut junction_boxes: BTreeMap<usize, &JunctionBox> =
            BTreeMap::new();
        let id: usize = junction_box.id;
        junction_boxes.insert(id, junction_box);
        Circuit {
            junction_boxes: junction_boxes,
        }
    }

    // returns the number of junction boxes in the circuit
    //
    fn junction_box_count(&self) -> usize {
        self.junction_boxes.len()
    }

    // Returns true if this Circuit object contains
    // a junction box with the given id; otherwise, false.
    //
    fn contains_junction_box(&self, id: usize) -> bool {
        self.junction_boxes.contains_key(&id)
    }

    // Add a reference to a junction box to this circuit,
    // if the circuit doesn't already contain it.
    //
    fn add(&mut self, junction_box: &'a JunctionBox) {
        if !self.contains_junction_box(junction_box.id) {
            let id: usize = junction_box.id;
            self.junction_boxes.insert(id, junction_box);
        }
    }
}

// find the two junction boxes that are closest,
// but farther than some minimum
//
// Returns ids of the boxes and the distance.
//
fn find_closest_pairs(
    junction_boxes: &Vec<JunctionBox>,
    already_paired: &BTreeMap<usize, BTreeSet<usize>>,
    min_dist: u64,
) -> (usize, usize, u64) {
    let mut closest_distance = u64::MAX;
    let mut closest_idx_a = usize::MAX;
    let mut closest_idx_b = usize::MAX;
    let len = junction_boxes.len();
    find_closest_pair(
        junction_boxes,
        already_paired,
        0..len,
        &mut closest_distance,
        &mut closest_idx_a,
        &mut closest_idx_b,
        min_dist,
    );
    (closest_idx_a, closest_idx_b, closest_distance)
}

// over the given range, find the closest boxes
//
fn find_closest_pair(
    junction_boxes: &Vec<JunctionBox>,
    already_paired: &BTreeMap<usize, BTreeSet<usize>>,
    rng: Range<usize>,
    closest_distance: &mut u64,
    closest_idx_a: &mut usize,
    closest_idx_b: &mut usize,
    min_dist: u64,
) {
    println!("find_closest_pair(.., {}..{}, ...)", rng.start, rng.end);
    let idx: usize = rng.start;
    let end: usize = rng.end;
    if 1 >= (end - idx) {
        return;
    }
    let start = idx + 1;
    find_closest_pair(
        junction_boxes,
        already_paired,
        start..end,
        closest_distance,
        closest_idx_a,
        closest_idx_b,
        min_dist,
    );
    for other_idx in start..end {
        let a: &JunctionBox = &(junction_boxes[idx]);
        let b: &JunctionBox = &(junction_boxes[other_idx]);
        // make sure we haven't already paired these
        //
        if already_paired.contains_key(&idx) {
            let paired_with = already_paired.get(&idx).unwrap();
            if paired_with.contains(&other_idx) {
                continue;
            }
        }
        let d = a.distance_from(b);
        if (d >= min_dist) && (d < *closest_distance) {
            *closest_distance = d;
            *closest_idx_a = a.id;
            *closest_idx_b = b.id;
            println!(
                "close: {}, {}, {}",
                closest_idx_a, closest_idx_b, closest_distance
            );
        }
    }
}

fn add_pair(
    already_paired: &mut BTreeMap<usize, BTreeSet<usize>>,
    id_a: usize,
    id_b: usize,
) {
    if !already_paired.contains_key(&id_a) {
        let paired_with: BTreeSet<usize> = BTreeSet::new();
        already_paired.insert(id_a, paired_with);
    }
    if !already_paired.contains_key(&id_b) {
        let paired_with: BTreeSet<usize> = BTreeSet::new();
        already_paired.insert(id_b, paired_with);
    }
    let paired_with = already_paired.get_mut(&id_a).unwrap();
    if (!paired_with.contains(&id_b)) {
        paired_with.insert(id_b);
    }
    let paired_with = already_paired.get_mut(&id_b).unwrap();
    if (!paired_with.contains(&id_a)) {
        paired_with.insert(id_a);
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
    let a = JunctionBox::new(162, 187, 812, 0);
    let b = JunctionBox::new(425, 690, 689, 1);
    let dist = a.distance_from(&b);
    assert_eq!(337307, dist);
}

#[test]
fn check_distance_2() {
    let a = JunctionBox::new(739, 650, 466, 0);
    let b = JunctionBox::new(346, 949, 466, 1);
    let dist = a.distance_from(&b);
    assert_eq!(243850, dist);
}

// test with example input, one pass
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
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut circuits_by_id: BTreeMap<usize, usize> = BTreeMap::new();
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
        let x = xs.parse::<i64>().unwrap();
        let ys = coords.get(2).unwrap().as_str();
        let y = ys.parse::<i64>().unwrap();
        let zs = coords.get(3).unwrap().as_str();
        let z = zs.parse::<i64>().unwrap();
        let junction_box: JunctionBox = JunctionBox::new(x, y, z, idx);
        junction_boxes.push(junction_box);
        idx += 1;
    }
    println!("Read in {} points", junction_boxes.len());

    // find the n closest junction boxeds
    //
    let mut min_dist = 0_u64;
    let mut connection_count: usize = 0;
    let mut already_paired: BTreeMap<usize, BTreeSet<usize>> =
        BTreeMap::new();
    while connection_count < upto {
        let (id_a, id_b, dist) = find_closest_pairs(
            &junction_boxes,
            &already_paired,
            min_dist,
        );
        min_dist = dist;
        let a_in_circuit = circuits_by_id.contains_key(&id_a);
        let b_in_circuit = circuits_by_id.contains_key(&id_b);
        if a_in_circuit {
            let cid_a = *circuits_by_id.get(&id_a).unwrap();
            let circuit_a = circuits.get_mut(cid_a).unwrap();
            if b_in_circuit {
                let cid_b = *circuits_by_id.get(&id_b).unwrap();
                if cid_a == cid_b {
                    // both boxes are in the same circuit. Count that
                    // as a connection.
                    //
                    connection_count += 1;
                    add_pair(&mut already_paired, id_a, id_b);
                    println!(
                        "RE-Connecting {} and {}",
                        junction_boxes
                            .get(id_a)
                            .unwrap()
                            .describe_coords(),
                        junction_boxes
                            .get(id_b)
                            .unwrap()
                            .describe_coords()
                    );
                    println!(
                        "Circuit {} has {} junction boxes",
                        cid_a,
                        circuit_a.junction_box_count()
                    );
                } else {
                    // each box is in a different circuit;
                    // don't count that as making a connection
                    //
                    connection_count += 1;
                    add_pair(&mut already_paired, id_a, id_b);
                    println!(
                        "Circuits {} and {} are unchanged",
                        cid_a, cid_b
                    );
                }
            } else {
                circuit_a.add(junction_boxes.get(id_b).unwrap());
                circuits_by_id.insert(id_b, cid_a);
                connection_count += 1;
                add_pair(&mut already_paired, id_a, id_b);
                println!(
                    "Connecting {} and {}",
                    junction_boxes.get(id_a).unwrap().describe_coords(),
                    junction_boxes.get(id_b).unwrap().describe_coords()
                );
                println!(
                    "Circuit {} has {} junction boxes",
                    cid_a,
                    circuit_a.junction_box_count()
                );
            }
        } else if b_in_circuit {
            let cid_b = *circuits_by_id.get(&id_b).unwrap();
            let circuit_b = circuits.get_mut(cid_b).unwrap();
            circuit_b.add(junction_boxes.get(id_a).unwrap());
            circuits_by_id.insert(id_a, cid_b);
            connection_count += 1;
            add_pair(&mut already_paired, id_a, id_b);
            println!(
                "Connecting {} and {}",
                junction_boxes.get(id_a).unwrap().describe_coords(),
                junction_boxes.get(id_b).unwrap().describe_coords()
            );
            println!(
                "Circuit {} has {} junction boxes",
                cid_b,
                circuit_b.junction_box_count()
            );
        } else {
            let mut circuit_new: Circuit =
                Circuit::new(junction_boxes.get(id_a).unwrap());
            circuit_new.add(junction_boxes.get(id_b).unwrap());
            circuits.push(circuit_new);
            let cid_new = circuits.len() - 1;
            circuits_by_id.insert(id_a, cid_new);
            circuits_by_id.insert(id_b, cid_new);
            connection_count += 1;
            add_pair(&mut already_paired, id_a, id_b);
            println!(
                "Connecting {} and {}",
                junction_boxes.get(id_a).unwrap().describe_coords(),
                junction_boxes.get(id_b).unwrap().describe_coords()
            );
            println!(
                "Circuit {} has {} junction boxes",
                cid_new,
                circuits.get(cid_new).unwrap().junction_box_count()
            );
        }
    }

    // sort circuits by size and id
    //
    let mut largest_circuits: Vec<usize> =
        (0..circuits.len()).collect();
    println!("largest_circuits length is {}", largest_circuits.len());
    largest_circuits.sort_by(|a, b| {
        let c_a: &Circuit = circuits.get(*a).unwrap();
        let c_b: &Circuit = circuits.get(*b).unwrap();
        if c_a.junction_box_count() > c_b.junction_box_count() {
            Ordering::Less
        } else if c_a.junction_box_count() < c_b.junction_box_count() {
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
            circuits[*i].junction_box_count()
        );
    }
    for i in 0..productoflargest {
        actual_product *= circuits
            .get(largest_circuits[i])
            .unwrap()
            .junction_box_count();
    }
    assert_eq!(expected_product, actual_product);
}
