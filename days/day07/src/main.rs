use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the problem set,
/// solve the problems and accumulate the answers.ingredient database,
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

#[derive(Debug)]
enum Equipment {
    Empty,
    Splitter,
    Start,
}

#[derive(Debug)]
struct EquipmentConfig {
    config: Vec<Equipment>,
    has_start: bool,
    start_idx: usize,
}

impl EquipmentConfig {
    // constructor
    //
    fn new() -> Self {
        let config: Vec<Equipment> = Vec::new();
        EquipmentConfig {
            config: config,
            has_start: false,
            start_idx: usize::MAX,
        }
    }

    fn has_splitter_at(&self, idx: usize) -> bool {
        match self.config.get(idx) {
            None => false,
            Some(e) => match e {
                Equipment::Splitter => true,
                _ => false,
            },
        }
    }

    // Returns true if the configuration contains a beam entry point;
    // otherwise, returns false.
    //
    fn has_start(&self) -> bool {
        self.has_start
    }

    // parse an input line into a set of equipment
    //
    fn into_equipment(&mut self, line: &str) {
        if 0 < self.config.len() {
            panic!("already configured; cannot reconfigure");
        }
        for c in line.chars() {
            match c {
                '.' => {
                    self.config.push(Equipment::Empty);
                }
                '^' => {
                    self.config.push(Equipment::Splitter);
                }
                'S' => {
                    self.config.push(Equipment::Start);
                    self.has_start = true;
                    self.start_idx = self.config.len() - 1;
                }
                _ => {}
            }
        }
    }

    // length of the equipment list
    //
    fn len(&self) -> usize {
        self.config.len()
    }

    // if the configuration includes the beam entry point,
    // return the index of the entry point position.
    //
    fn start_at(&self) -> usize {
        if !self.has_start {
            panic!(
                "Equipment configuration does not have a beam entry point"
            );
        }
        self.start_idx
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

    let mut split_count: i64 = 0;
    let mut started: bool = false;
    let mut incoming_beams: BTreeSet<usize> = BTreeSet::new();
    for line in lines {
        let mut outgoing_beams: BTreeSet<usize> = BTreeSet::new();
        let line = line.unwrap();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        let mut equip: EquipmentConfig = EquipmentConfig::new();
        equip.into_equipment(line);
        if !started && equip.has_start() {
            outgoing_beams.insert(equip.start_at());
            started = true;
        } else if started {
            if equip.has_start() {
                panic!("multiple beam entry points!");
            }
            let equip_count = equip.len();
            for beam_idx in incoming_beams.iter() {
                let beam_idx = *beam_idx;
                if equip.has_splitter_at(beam_idx) {
                    split_count += 1;
                    if beam_idx > 0 {
                        let i = beam_idx - 1;
                        if !outgoing_beams.contains(&i) {
                            outgoing_beams.insert(i);
                        }
                    }
                    if beam_idx < (equip_count - 1) {
                        let i = beam_idx + 1;
                        if !outgoing_beams.contains(&i) {
                            outgoing_beams.insert(i);
                        }
                    }
                } else {
                    outgoing_beams.insert(beam_idx);
                }
            }
        }
        incoming_beams = outgoing_beams;
    }
    if !started {
        panic!("NOT STARTED!!");
    }

    // Display the grand total of problem answers
    //
    println!("The beam split count is {}", split_count);
    Ok(())
}

// test with example input
//
#[test]
fn given_example() {
    let expected_split_count: i64 = 21;
    let raw_input = " .......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
    .to_string();
    let mut actual_split_count: i64 = 0;
    let mut started: bool = false;
    let mut incoming_beams: BTreeSet<usize> = BTreeSet::new();
    let input = raw_input.as_str();
    let lines = input.split('\n');
    for line in lines {
        let mut outgoing_beams: BTreeSet<usize> = BTreeSet::new();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        let mut equip: EquipmentConfig = EquipmentConfig::new();
        equip.into_equipment(line);
        if !started && equip.has_start() {
            outgoing_beams.insert(equip.start_at());
            started = true;
        } else if started {
            if equip.has_start() {
                panic!("multiple beam entry points!");
            }
            let equip_count = equip.len();
            for beam_idx in incoming_beams.iter() {
                let beam_idx = *beam_idx;
                if equip.has_splitter_at(beam_idx) {
                    actual_split_count += 1;
                    if beam_idx > 0 {
                        let i = beam_idx - 1;
                        if !outgoing_beams.contains(&i) {
                            outgoing_beams.insert(i);
                        }
                    }
                    if beam_idx < (equip_count - 1) {
                        let i = beam_idx + 1;
                        if !outgoing_beams.contains(&i) {
                            outgoing_beams.insert(i);
                        }
                    }
                } else {
                    outgoing_beams.insert(beam_idx);
                }
            }
        }
        incoming_beams = outgoing_beams;
    }
    if !started {
        panic!("NOT STARTED!!");
    }
    assert_eq!(expected_split_count, actual_split_count);
}
