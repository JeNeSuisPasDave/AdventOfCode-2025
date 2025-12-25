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
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }

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

#[derive(Debug, Clone, Copy)]
enum TileColor {
    Red,
    Green,
    Other,
}

#[derive(Debug)]
struct Tile {
    loc: Point,
    color: TileColor,
}

impl Tile {
    fn new(loc: Point, color: TileColor) -> Self {
        Tile {
            loc: loc,
            color: color,
        }
    }
}

struct TileGrid {
    tiles: BTreeMap<u64, BTreeMap<u64, Tile>>,
    min_x: u64,
    min_y: u64,
    max_x: u64,
    max_y: u64,
}

impl TileGrid {
    fn new() -> Self {
        let grid: BTreeMap<u64, BTreeMap<u64, Tile>> = BTreeMap::new();
        TileGrid {
            tiles: grid,
            min_x: u64::MAX,
            min_y: u64::MAX,
            max_x: 0,
            max_y: 0,
        }
    }

    fn insert_green_tile(&mut self, loc: &Point) {
        self.insert_tile(loc, TileColor::Green);
    }

    fn insert_red_tile(&mut self, loc: &Point) {
        self.insert_tile(loc, TileColor::Red);
    }

    fn insert_tile(&mut self, loc: &Point, color: TileColor) {
        match color {
            TileColor::Red => {}
            TileColor::Green => {}
            _ => {
                panic!("Unexpected tile color")
            }
        }
        if !self.tiles.contains_key(&loc.x) {
            let row: BTreeMap<u64, Tile> = BTreeMap::new();
            self.tiles.insert(loc.x, row);
        }
        let row = self.tiles.get_mut(&loc.x).unwrap();
        if !row.contains_key(&loc.y) {
            let tile = Tile::new(loc.clone(), color);
            row.insert(loc.y, tile);
            self.min_x = self.min_x.min(loc.x);
            self.min_y = self.min_y.min(loc.y);
            self.max_x = self.max_x.max(loc.x);
            self.max_y = self.max_y.max(loc.y);
        }
        // check the insertion
        //
        if !self.tiles.contains_key(&loc.x) {
            panic!("Missing x");
        }
        let row = self.tiles.get(&loc.x).unwrap();
        if !row.contains_key(&loc.y) {
            panic!("missing y")
        }
    }

    fn connect_red_tiles_with_green_tiles(
        &mut self,
        a: &Point,
        b: &Point,
    ) {
        if a.x == b.x {
            // draw up or down
            //
            let x = a.x;
            if a.y <= b.y {
                let start = a.y + 1;
                let end = b.y;
                for y in start..end {
                    let loc = Point::new(x, y);
                    self.insert_green_tile(&loc);
                }
            } else {
                let start = b.y + 1;
                let end = a.y;
                for y in start..end {
                    let loc = Point::new(x, y);
                    self.insert_green_tile(&loc);
                }
            }
        } else {
            // draw left or right
            //
            let y = a.y;
            if a.x <= b.x {
                let start = a.x + 1;
                let end = b.x;
                for x in start..end {
                    let loc = Point::new(x, y);
                    self.insert_green_tile(&loc);
                }
            } else {
                let start = b.x + 1;
                let end = a.x;
                for x in start..end {
                    let loc = Point::new(x, y);
                    self.insert_green_tile(&loc);
                }
            }
        }
    }

    fn fill_in_loops(&mut self) {
        for y in self.min_y..=self.max_y {
            let is_outside = true;
            for x in self.min_x..=self.max_x {}
        }
    }

    fn get_color(&self, x: u64, y: u64) -> TileColor {
        if !self.tiles.contains_key(&x) {
            TileColor::Other
        } else {
            let row = self.tiles.get(&x).unwrap();
            if !row.contains_key(&y) {
                TileColor::Other
            } else {
                row.get(&y).unwrap().color
            }
        }
    }

    fn display_grid(&self) {
        for y in 0..=self.max_y {
            let mut disp_row: Vec<String> = Vec::new();
            for x in 0..=self.max_x {
                let color = self.get_color(x, y);
                match color {
                    TileColor::Red => {
                        disp_row.push("#".to_string());
                    }
                    TileColor::Green => {
                        disp_row.push("X".to_string());
                    }
                    TileColor::Other => {
                        disp_row.push(".".to_string());
                    }
                }
            }
            println!("{}", disp_row.join(""));
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

#[test]
fn t_given_example_part2() {
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

    let mut grid = TileGrid::new();

    let len = points.len();
    for i in 0..len {
        let p: &Point = points.get(i).unwrap();
        println!("About to insert ({},{})", p.x, p.y);
        grid.insert_red_tile(points.get(i).unwrap());
    }
    let mut a = 0;
    for next in 1..=len {
        let mut b = next;
        if next == len {
            b = 0;
        }
        grid.connect_red_tiles_with_green_tiles(
            points.get(a).unwrap(),
            points.get(b).unwrap(),
        );
        a = b;
    }

    grid.display_grid();

    assert_eq!(0, 2);
}
