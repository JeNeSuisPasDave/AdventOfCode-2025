use ::std::cmp::Ordering;
use ::std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Id, Parser};
use regex::Regex;

/// Given input file containing the problem set,
/// repeatedly connect the next closest junction boxes,
/// until the specified number of connection attempts were made.
/// If all boxes are connected, return the product of the last
/// two connected box's X coordinates; otherwise return the
/// product of the sizes of the largest n circuits, where
/// n is specified with the --product-terms argument value.
///
#[derive(Parser, Debug)]
struct Cli {
    /// whether to connect all junction boxes
    #[arg(long = "connect-all")]
    connectall: bool,
    /// the maximum number of circuits to assemble,
    /// default 10
    #[arg(short = 'c', long = "connection-attempts")]
    upto: Option<usize>,
    /// the number of the largest circuits from which
    /// to produce the product of their sizes, default 3
    #[arg(short = 'p', long = "product-terms")]
    productoflargest: Option<usize>,
    /// The path to the file containing battery bank specs
    path: PathBuf,
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

struct JunctionBoxPair {
    first_box_id: usize,
    second_box_id: usize,
    distance: u64,
}

impl JunctionBoxPair {
    fn new(a: usize, b: usize, dist: u64) -> Self {
        if a == b {
            panic!("a is the same as b");
        }
        if a < b {
            JunctionBoxPair {
                first_box_id: a,
                second_box_id: b,
                distance: dist,
            }
        } else {
            JunctionBoxPair {
                first_box_id: b,
                second_box_id: a,
                distance: dist,
            }
        }
    }
}

struct Circuit {
    jbs: BTreeSet<usize>,
    id: usize,
}

impl Circuit {
    fn new(id: usize) -> Self {
        let jbs: BTreeSet<usize> = BTreeSet::new();
        Circuit { jbs: jbs, id: id }
    }

    fn contains(&self, junction_box_id: usize) -> bool {
        self.jbs.contains(&junction_box_id)
    }

    fn describe_circuit(&self) -> String {
        let l: Vec<String> =
            self.jbs.iter().map(|x| x.to_string()).collect();
        l.join(",")
    }

    fn insert_box(&mut self, junction_box_id: usize) {
        if !self.jbs.contains(&junction_box_id) {
            self.jbs.insert(junction_box_id);
        }
    }

    fn insert_circuit(&mut self, other: &Self) {
        for jb_id in other.jbs.iter() {
            self.insert_box(*jb_id);
        }
    }

    fn insert_list(&mut self, other_jbs: &Vec<usize>) {
        for jb_id in other_jbs.iter() {
            self.insert_box(*jb_id);
        }
    }

    fn insert_pair(&mut self, pair: &JunctionBoxPair) {
        self.insert_box(pair.first_box_id);
        self.insert_box(pair.second_box_id);
    }

    fn len(&self) -> usize {
        self.jbs.len()
    }
}

fn find_distances(
    junction_boxes: &Vec<JunctionBox>,
    pairs_by_first_id: &mut BTreeMap<
        usize,
        BTreeMap<usize, JunctionBoxPair>,
    >,
    rng: Range<usize>,
) {
    let id_a: usize = rng.start;
    let end: usize = rng.end;
    if 1 >= (end - id_a) {
        return;
    }
    let start = id_a + 1;
    find_distances(junction_boxes, pairs_by_first_id, start..end);
    for id_b in start..end {
        if pairs_by_first_id.contains_key(&id_a) {
            let paired_with = pairs_by_first_id.get(&id_a).unwrap();
            if paired_with.contains_key(&id_b) {
                continue;
            }
        }
        let dist: u64 =
            junction_boxes[id_a].distance_from(&junction_boxes[id_b]);
        let pair = JunctionBoxPair::new(id_a, id_b, dist);
        if !pairs_by_first_id.contains_key(&id_a) {
            let mut paired_with: BTreeMap<usize, JunctionBoxPair> =
                BTreeMap::new();
            paired_with.insert(id_b, pair);
            pairs_by_first_id.insert(id_a, paired_with);
        } else {
            let paired_with = pairs_by_first_id.get_mut(&id_a).unwrap();
            paired_with.insert(id_b, pair);
        }
    }
}

fn sort_pairs_by_distance(
    pairs: &BTreeMap<usize, BTreeMap<usize, JunctionBoxPair>>,
    list: &mut Vec<(usize, usize)>,
) {
    let mut local_list: Vec<(usize, usize, u64)> = Vec::new();
    for key_a in pairs.keys() {
        let paired_with = pairs.get(key_a).unwrap();
        for key_b in paired_with.keys() {
            let jb = paired_with.get(key_b).unwrap();
            local_list.push((*key_a, *key_b, jb.distance));
        }
    }
    local_list.sort_by(|a, b| {
        if a.2 > b.2 {
            Ordering::Greater
        } else if a.2 < b.2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    for (id_a, id_b, _) in local_list.iter() {
        list.push((*id_a, *id_b));
    }
}

fn build_circuits(
    upto: &usize,
    sorted_pairs: &Vec<(usize, usize)>,
    last_two: &mut (usize, usize),
    jb_count: usize,
) -> BTreeMap<usize, Circuit> {
    let mut next_id: usize = 0;
    let mut circuits: BTreeMap<usize, Circuit> = BTreeMap::new();
    let upto = usize::min(*upto, sorted_pairs.len());
    for pass in 0..upto {
        let (id_a, id_b) = sorted_pairs[pass];
        // println!("({}-{})", id_a, id_b);
        let circuit_ids: Vec<usize> =
            circuits.keys().map(|x| *x).collect();
        // if we have one circuit containing all the boxes,
        // then stop building
        //
        if 1 == circuit_ids.len() {
            let cid = circuit_ids.get(0).unwrap();
            let c = circuits.get(&cid).unwrap();
            if jb_count <= c.len() {
                break;
            }
        }
        let mut target_circuit_ids: Vec<usize> = Vec::new();
        for id in circuit_ids {
            let circuit = circuits.get_mut(&id).unwrap();
            if circuit.contains(id_a) || circuit.contains(id_b) {
                target_circuit_ids.push(id);
            }
        }
        if 0 == target_circuit_ids.len() {
            let mut new_circuit = Circuit::new(next_id);
            next_id += 1;
            new_circuit.insert_box(id_a);
            new_circuit.insert_box(id_b);
            circuits.insert(new_circuit.id, new_circuit);
            last_two.0 = id_a;
            last_two.1 = id_b;
        } else {
            // add the pair to the existing circuit
            //
            let target =
                circuits.get_mut(&target_circuit_ids[0]).unwrap();
            target.insert_box(id_a);
            target.insert_box(id_b);
            // does the pair reference another circuit?
            //
            if (1 == target_circuit_ids.len()) {
                last_two.0 = id_a;
                last_two.1 = id_b;
            } else if (1 < target_circuit_ids.len())
                && (target_circuit_ids[0] != target_circuit_ids[1])
            {
                // if so, then merge the two circuits
                //
                let other =
                    circuits.get(&target_circuit_ids[1]).unwrap();
                let other_jbs: Vec<usize> =
                    other.jbs.iter().map(|x| *x).collect();
                let target =
                    circuits.get_mut(&target_circuit_ids[0]).unwrap();
                target.insert_list(&other_jbs);
                circuits.remove(&target_circuit_ids[1]);
                last_two.0 = id_a;
                last_two.1 = id_b;
            }
        }
        // let mut bld: Vec<String> = Vec::new();
        // for c_id in circuits.keys() {
        //     let circuit = circuits.get(c_id).unwrap();
        //     bld.push(format!("[{}]", circuit.describe_circuit()));
        // }
        // println!("{}", bld.join(" "));
    }
    circuits
}

fn sort_circuits(
    circuits: &BTreeMap<usize, Circuit>,
) -> Vec<(usize, usize)> {
    let mut sorted_circuits: Vec<(usize, usize)> = Vec::new();
    for id in circuits.keys() {
        let c = circuits.get(id).unwrap();
        sorted_circuits.push((c.id, c.len()));
    }

    // sort in descending order by length
    //
    sorted_circuits.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Less
        } else if a.1 < b.1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    sorted_circuits
}

// Binary crate entry point
//
fn main() -> Result<()> {
    let args = Cli::parse();
    let mut upto: usize = 10;
    if let Some(x) = args.upto {
        upto = x;
    }
    let mut productoflargest: usize = 3;
    if let Some(x) = args.productoflargest {
        productoflargest = x;
    }
    let connect_all = args.connectall;
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let rdr = BufReader::new(f);
    let lines = rdr.lines();

    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    let re_coord =
        Regex::new(r"^\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*$")
            .unwrap();
    let mut line_num: usize = 0;
    let mut idx: usize = 0;
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
        let x = xs.parse::<i64>().unwrap();
        let ys = coords.get(2).unwrap().as_str();
        let y = ys.parse::<i64>().unwrap();
        let zs = coords.get(3).unwrap().as_str();
        let z = zs.parse::<i64>().unwrap();
        let junction_box: JunctionBox = JunctionBox::new(x, y, z, idx);
        junction_boxes.push(junction_box);
        idx += 1;
    }

    println!("found {} junction boxes", junction_boxes.len());

    // for jb in junction_boxes.iter() {
    //     println!("{}: {}", jb.id, jb.describe_coords());
    // }

    let len = junction_boxes.len();
    let mut pairs: BTreeMap<usize, BTreeMap<usize, JunctionBoxPair>> =
        BTreeMap::new();
    find_distances(&junction_boxes, &mut pairs, 0..len);

    // for key_a in pairs.keys() {
    //     let paired_with = pairs.get(key_a).unwrap();
    //     for key_b in paired_with.keys() {
    //         let jb = paired_with.get(key_b).unwrap();
    //         println!(
    //             "{}-{}: {}",
    //             jb.first_box_id, jb.second_box_id, jb.distance
    //         );
    //     }
    // }

    let mut sorted_pairs: Vec<(usize, usize)> = Vec::new();
    sort_pairs_by_distance(&pairs, &mut sorted_pairs);

    if connect_all {
        upto = usize::MAX;
    }
    println!("upto: {}", upto);

    // println!("SORTED:");
    // let mut count = 0;
    // for (key_a, key_b) in sorted_pairs.iter() {
    //     if count >= upto {
    //         break;
    //     }
    //     let jb = pairs.get(&key_a).unwrap().get(&key_b).unwrap();
    //     println!(
    //         "{}-{}: {}",
    //         jb.first_box_id, jb.second_box_id, jb.distance
    //     );
    //     count += 1;
    // }

    let mut last_two: (usize, usize) = (0, 0);
    let circuits = build_circuits(
        &upto,
        &sorted_pairs,
        &mut last_two,
        junction_boxes.len(),
    );

    // println!("CIRCUITS:");
    // for circuit_id in circuits.keys() {
    //     println!(
    //         "{}: {}",
    //         circuit_id,
    //         circuits[circuit_id].describe_circuit()
    //     );
    // }

    if connect_all {
        let product: u64 = u64::try_from(
            junction_boxes[last_two.0].location.x
                * junction_boxes[last_two.1].location.x,
        )
        .unwrap();
        println!(
            "Product of the x coord of last two boxes connected is {}",
            product
        );
    } else {
        let sorted_circuits = sort_circuits(&circuits);
        let mut product: u64 = 1;
        let limit: usize = productoflargest;
        for i in 0..limit {
            let len: u64 = u64::try_from(sorted_circuits[i].1).unwrap();
            product *= len;
        }
        println!(
            "Product of the largest {} circuits is {}",
            productoflargest, product
        );
    }

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
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
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
}
