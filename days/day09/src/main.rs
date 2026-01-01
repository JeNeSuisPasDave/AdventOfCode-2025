use ::std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

/// Given input file containing the coordinates of red tiles,
/// find the largest area bounded by red tiles as opposite corners.
///
#[derive(Parser, Debug)]
struct Cli {
    /// Whether to apply the green tile specifications
    #[arg(long = "consider-green-tiles")]
    with_green_tiles: bool,
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

    fn display(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum TileColor {
    Red,
    Green,
    GreenFill,
    Other,
}

#[derive(Debug, Clone, Copy)]
enum InsideIs {
    // red tile inside direction
    //
    UpperRight,
    UpperLeft,
    LowerLeft,
    LowerRight,
    NotUpperRight,
    NotUpperLeft,
    NotLowerLeft,
    NotLowerRight,

    // green tile indisde direction
    //
    Above,
    Left,
    Below,
    Right,
    Unknown,
}

#[derive(Debug)]
struct Tile {
    loc: Point,
    color: TileColor,
    inside_direction: InsideIs,
}

impl Tile {
    fn new(loc: Point, color: TileColor) -> Self {
        Tile {
            loc: loc,
            color: color,
            inside_direction: InsideIs::Unknown,
        }
    }

    fn set_inside_direction(&mut self, inside_direction: InsideIs) {
        match self.inside_direction {
            InsideIs::Unknown => {
                self.inside_direction = inside_direction;
            }
            _ => {}
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

    fn insert_green_fill_tile(&mut self, loc: &Point) {
        self.insert_tile(loc, TileColor::GreenFill);
    }

    fn insert_red_tile(&mut self, loc: &Point) {
        self.insert_tile(loc, TileColor::Red);
    }

    fn insert_tile(&mut self, loc: &Point, color: TileColor) {
        match color {
            TileColor::Red => {}
            TileColor::Green => {}
            TileColor::GreenFill => {}
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

    fn mark_red_tiles_moving_down(
        &mut self,
        a_inside_dir: &InsideIs,
        a: &Point,
        b: &Point,
    ) {
        let x = a.x;
        match a_inside_dir {
            InsideIs::NotLowerLeft => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperRight,
                    );
                } else {
                    panic!("DOWN 1");
                }
            }
            InsideIs::LowerLeft => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperRight,
                    );
                } else {
                    panic!("DOWN 2");
                }
            }
            InsideIs::NotLowerRight => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperRight,
                    );
                } else {
                    panic!("DOWN 3");
                }
            }
            InsideIs::LowerRight => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperRight,
                    );
                } else {
                    panic!("DOWN 4");
                }
            }
            _ => {
                panic!(
                    "Unexpected DOWN a_inside_dir: {:?}",
                    a_inside_dir
                );
            }
        }
    }

    fn mark_red_tiles_moving_up(
        &mut self,
        a_inside_dir: &InsideIs,
        a: &Point,
        b: &Point,
    ) {
        let x = a.x;
        match a_inside_dir {
            InsideIs::NotUpperLeft => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerRight,
                    );
                } else {
                    panic!("UP 1");
                }
            }
            InsideIs::UpperLeft => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerRight,
                    );
                } else {
                    panic!("UP 2");
                }
            }
            InsideIs::UpperRight => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerRight,
                    );
                } else {
                    panic!("UP 3");
                }
            }
            InsideIs::NotUpperRight => {
                if self.is_color_other(x + 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerLeft,
                    );
                } else if self.is_color_other(x - 1, b.y) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerRight,
                    );
                } else {
                    panic!("UP 4");
                }
            }
            _ => {
                panic!("Unexpected UP a_inside_dir");
            }
        }
    }

    fn mark_red_tiles_moving_right(
        &mut self,
        a_inside_dir: &InsideIs,
        a: &Point,
        b: &Point,
    ) {
        let y = a.y;
        match a_inside_dir {
            InsideIs::NotLowerRight => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerLeft,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperLeft,
                    );
                } else {
                    panic!("RIGHT 1");
                }
            }
            InsideIs::LowerRight => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerLeft,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperLeft,
                    );
                } else {
                    panic!("RIGHT 2");
                }
            }
            InsideIs::NotUpperRight => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerLeft,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperLeft,
                    );
                } else {
                    panic!("RIGHT 3");
                }
            }
            InsideIs::UpperRight => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerLeft,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperLeft,
                    );
                } else {
                    panic!("RIGHT 4");
                }
            }
            _ => {
                panic!("Unexpected RIGHT a_inside_dir");
            }
        }
    }

    fn mark_red_tiles_moving_left(
        &mut self,
        a_inside_dir: &InsideIs,
        a: &Point,
        b: &Point,
    ) {
        let y = a.y;
        match a_inside_dir {
            InsideIs::NotLowerLeft => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerRight,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperRight,
                    );
                } else {
                    panic!("LEFT 1");
                }
            }
            InsideIs::UpperLeft => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotLowerRight,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::UpperRight,
                    );
                } else {
                    panic!("LEFT 2");
                }
            }
            InsideIs::NotUpperLeft => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerRight,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperRight,
                    );
                } else {
                    panic!("LEFT 3");
                }
            }
            InsideIs::LowerLeft => {
                if self.is_color_other(b.x, y - 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::LowerRight,
                    );
                } else if self.is_color_other(b.x, y + 1) {
                    self.set_inside_direction(
                        b.x,
                        b.y,
                        InsideIs::NotUpperRight,
                    );
                } else {
                    panic!("LEFT 4");
                }
            }
            _ => {
                panic!("Unexpected LEFT a_inside_dir");
            }
        }
    }

    fn mark_red_tiles_with_inside_direction(
        &mut self,
        a: &Point,
        b: &Point,
    ) {
        let mut a_inside_dir = self.get_inside_direction(a.x, a.y);
        let b_inside_dir = self.get_inside_direction(b.x, b.y);
        match a_inside_dir {
            InsideIs::Unknown => {
                let dir = self.find_inside_direction(a.x, a.y);
                self.set_inside_direction(a.x, a.y, dir);
                a_inside_dir = self.get_inside_direction(a.x, a.y);
            }
            _ => {}
        }
        match b_inside_dir {
            InsideIs::Unknown => {
                if a.x == b.x {
                    // moving down or up
                    //
                    if a.y < b.y {
                        // moving down
                        //
                        self.mark_red_tiles_moving_down(
                            &a_inside_dir,
                            a,
                            b,
                        );
                    } else {
                        // moving up
                        //
                        self.mark_red_tiles_moving_up(
                            &a_inside_dir,
                            a,
                            b,
                        );
                    }
                } else if a.y == b.y {
                    // moving left or right
                    //
                    let y = a.y;
                    if a.x < b.x {
                        // moving right
                        //
                        self.mark_red_tiles_moving_right(
                            &a_inside_dir,
                            a,
                            b,
                        );
                    } else {
                        // moving left
                        //
                        self.mark_red_tiles_moving_left(
                            &a_inside_dir,
                            a,
                            b,
                        );
                    }
                } else {
                    panic!("Diagonal connection of red tiles ZZZ");
                }
            }
            _ => {}
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

    fn count_left(&self, x: u64, y: u64) -> u64 {
        let mut count: u64 = 0;
        let mut looking_for_red = false;
        let start = 0;
        let end = x;
        for i in start..end {
            match self.get_color(i, y) {
                TileColor::Other => {}
                TileColor::GreenFill => {}
                TileColor::Green => {
                    if !looking_for_red {
                        count += 1;
                    }
                }
                TileColor::Red => {
                    if !looking_for_red {
                        looking_for_red = true;
                        count += 1;
                    } else {
                        count += 1;
                        looking_for_red = false;
                    }
                }
            }
        }
        count
    }

    fn count_right(&self, x: u64, y: u64) -> u64 {
        let mut count: u64 = 0;
        let mut looking_for_red = false;
        let start = x + 1;
        let end = self.max_x + 1;
        for i in start..end {
            match self.get_color(i, y) {
                TileColor::Other => {}
                TileColor::GreenFill => {}
                TileColor::Green => {
                    if !looking_for_red {
                        count += 1;
                    }
                }
                TileColor::Red => {
                    if !looking_for_red {
                        looking_for_red = true;
                        count += 1;
                    } else {
                        count += 1;
                        looking_for_red = false;
                    }
                }
            }
        }
        count
    }

    fn count_up(&self, x: u64, y: u64) -> u64 {
        let mut count: u64 = 0;
        let mut looking_for_red = false;
        let start = 0;
        let end = y;
        for i in start..end {
            match self.get_color(x, i) {
                TileColor::Other => {}
                TileColor::GreenFill => {}
                TileColor::Green => {
                    if !looking_for_red {
                        count += 1;
                    }
                }
                TileColor::Red => {
                    if !looking_for_red {
                        looking_for_red = true;
                        count += 1;
                    } else {
                        count += 1;
                        looking_for_red = false;
                    }
                }
            }
        }
        count
    }

    fn count_down(&self, x: u64, y: u64) -> u64 {
        let mut count: u64 = 0;
        let mut looking_for_red = false;
        let start = y + 1;
        let end = self.max_y + 1;
        for i in start..end {
            match self.get_color(x, i) {
                TileColor::Other => {}
                TileColor::GreenFill => {}
                TileColor::Green => {
                    if !looking_for_red {
                        count += 1;
                    }
                }
                TileColor::Red => {
                    if !looking_for_red {
                        looking_for_red = true;
                        count += 1;
                    } else {
                        count += 1;
                        looking_for_red = false;
                    }
                }
            }
        }
        count
    }

    fn is_color_green(&self, x: u64, y: u64) -> bool {
        match self.get_color(x, y) {
            TileColor::Green => true,
            _ => false,
        }
    }

    fn is_color_green_fill(&self, x: u64, y: u64) -> bool {
        match self.get_color(x, y) {
            TileColor::GreenFill => true,
            _ => false,
        }
    }

    fn is_color_other(&self, x: u64, y: u64) -> bool {
        match self.get_color(x, y) {
            TileColor::Other => true,
            _ => false,
        }
    }

    fn is_color_red(&self, x: u64, y: u64) -> bool {
        match self.get_color(x, y) {
            TileColor::Red => true,
            _ => false,
        }
    }

    fn fill_if_neighbors(&mut self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self.get_color(x, y) {
                    TileColor::Other => {
                        if (self.min_x < x)
                            && (self.is_color_green_fill(x - 1, y))
                        {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        if (self.max_x > x)
                            && (self.is_color_green_fill(x + 1, y))
                        {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        if (self.min_y < y)
                            && (self.is_color_green_fill(x, y - 1))
                        {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        if (self.max_y > y)
                            && (self.is_color_green_fill(x, y + 1))
                        {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn fill_in_loops(&mut self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self.get_color(x, y) {
                    TileColor::Other => {
                        let c = self.count_left(x, y);
                        if 0 == c {
                            continue;
                        }
                        let c = self.count_right(x, y);
                        if 0 == c {
                            continue;
                        }
                        let c = self.count_up(x, y);
                        if 0 == c {
                            continue;
                        }
                        let c = self.count_down(x, y);
                        if 0 == c {
                            continue;
                        }
                        let c = self.count_left(x, y);
                        if 1 == (c % 2) {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        let c = self.count_right(x, y);
                        if 1 == (c % 2) {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        let c = self.count_up(x, y);
                        if 1 == (c % 2) {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                        let c = self.count_down(x, y);
                        if 1 == (c % 2) {
                            let loc = Point::new(x, y);
                            self.insert_green_fill_tile(&loc);
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.fill_if_neighbors();
    }

    fn is_outside(&self, x: u64, y: u64) -> bool {
        match self.get_color(x, y) {
            TileColor::Other => {
                let c = self.count_left(x, y);
                if 0 == c {
                    return true;
                }
                let c = self.count_right(x, y);
                if 0 == c {
                    return true;
                }
                let c = self.count_up(x, y);
                if 0 == c {
                    return true;
                }
                let c = self.count_down(x, y);
                if 0 == c {
                    return true;
                }
                let c = self.count_left(x, y);
                if 1 == (c % 2) {
                    return false;
                }
                let c = self.count_right(x, y);
                if 1 == (c % 2) {
                    return false;
                }
                let c = self.count_up(x, y);
                if 1 == (c % 2) {
                    return false;
                }
                let c = self.count_down(x, y);
                if 1 == (c % 2) {
                    return false;
                }
                return true;
            }
            _ => {
                return true;
            }
        }
    }

    fn find_inside_direction(&self, x: u64, y: u64) -> InsideIs {
        // only works for red tiles that are corner tiles
        //
        let upper_right_out;
        let upper_left_out;
        let lower_left_out;
        let lower_right_out;
        if 0 < x {
            if 0 < y {
                upper_left_out = self.is_outside(x - 1, y - 1);
                upper_right_out = self.is_outside(x + 1, y - 1);
                lower_right_out = self.is_outside(x + 1, y + 1);
                lower_left_out = self.is_outside(x - 1, y + 1);
            } else {
                // y == 0
                upper_left_out = true;
                upper_right_out = true;
                lower_right_out = self.is_outside(x + 1, y + 1);
                lower_left_out = self.is_outside(x - 1, y + 1);
            }
        } else {
            if 0 < y {
                // x == 0
                upper_left_out = true;
                upper_right_out = self.is_outside(x + 1, y - 1);
                lower_right_out = self.is_outside(x + 1, y + 1);
                lower_left_out = true;
            } else {
                // x == 0, y == 0
                upper_left_out = true;
                upper_right_out = true;
                lower_right_out = self.is_outside(x + 1, y + 1);
                lower_left_out = true;
            }
        }

        if upper_left_out && lower_left_out && lower_right_out {
            return InsideIs::UpperRight;
        }
        if upper_right_out && lower_left_out && lower_right_out {
            return InsideIs::UpperLeft;
        }
        if upper_right_out && upper_left_out && lower_right_out {
            return InsideIs::LowerLeft;
        }
        if upper_right_out && upper_left_out && lower_left_out {
            return InsideIs::LowerRight;
        }
        if upper_right_out {
            return InsideIs::NotUpperRight;
        }
        if upper_left_out {
            return InsideIs::NotUpperLeft;
        }
        if lower_left_out {
            return InsideIs::NotLowerLeft;
        }
        if lower_right_out {
            return InsideIs::NotLowerRight;
        }
        return InsideIs::Unknown;
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

    fn get_inside_direction(&self, x: u64, y: u64) -> InsideIs {
        if !self.tiles.contains_key(&x) {
            InsideIs::Unknown
        } else {
            let row = self.tiles.get(&x).unwrap();
            if !row.contains_key(&y) {
                InsideIs::Unknown
            } else {
                row.get(&y).unwrap().inside_direction
            }
        }
    }

    fn set_inside_direction(&mut self, x: u64, y: u64, idir: InsideIs) {
        if !self.tiles.contains_key(&x) {
            return;
        }
        let row = self.tiles.get_mut(&x).unwrap();
        if !row.contains_key(&y) {
            return;
        }
        row.get_mut(&y).unwrap().set_inside_direction(idir);
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
                    TileColor::GreenFill => {
                        disp_row.push("@".to_string());
                    }
                    TileColor::Other => {
                        disp_row.push(".".to_string());
                    }
                }
            }
            println!("{}", disp_row.join(""));
        }
    }

    fn is_filled(&self, a: &Point, b: &Point) -> bool {
        let mut ul: Point = Point::new(0, 0);
        let mut br: Point = Point::new(0, 0);
        if a.x < b.x && a.y < b.y {
            (ul.x, ul.y) = (a.x, a.y);
            (br.x, br.y) = (b.x, b.y);
        } else if a.x < b.x && a.y > b.y {
            (ul.x, ul.y) = (a.x, b.y);
            (br.x, br.y) = (b.x, a.y);
        } else if a.x > b.x && a.y < b.y {
            (ul.x, ul.y) = (b.x, a.y);
            (br.x, br.y) = (a.x, b.y);
        } else if a.x > b.x && a.y > b.y {
            (ul.x, ul.y) = (b.x, b.y);
            (br.x, br.y) = (a.x, a.y);
        }
        let x_s = ul.x + 1;
        let x_e = br.x;
        let y_s = ul.y + 1;
        let y_e = br.y;
        for x in x_s..x_e {
            for y in y_s..y_e {
                match self.get_color(x, y) {
                    TileColor::GreenFill => {}
                    _ => {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn find_max_filled_area(
        &self,
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
        self.find_max_filled_area(max_area, points, start..end);
        let point_a = points.get(id_a).unwrap();
        for id_b in start..end {
            let point_b = points.get(id_b).unwrap();
            if self.is_filled(point_a, point_b) {
                let area = point_a.area_with(point_b);
                if area > *max_area {
                    *max_area = area
                }
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
    let consider_green_tiles = &args.with_green_tiles;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;

    let now = Instant::now();
    let points = file_to_points(f);
    println!(
        "file_to_points() took {} secs",
        now.elapsed().as_secs_f64()
    );

    if !*consider_green_tiles {
        let mut max_area: u64 = 0;
        let len = points.len();
        let now = Instant::now();
        find_max_area(&mut max_area, &points, 0..len);
        println!(
            "find_max_area() took {} secs",
            now.elapsed().as_secs_f64()
        );

        println!("Max area: {}", max_area);
    } else {
        let mut grid = TileGrid::new();

        let now = Instant::now();
        let len = points.len();
        for i in 0..len {
            let p: &Point = points.get(i).unwrap();
            grid.insert_red_tile(points.get(i).unwrap());
        }
        println!(
            "inserting red tiles took {} secs",
            now.elapsed().as_secs_f64()
        );

        let now = Instant::now();
        let mut a = 0;
        for next in 1..=len {
            let mut b = next;
            if next == len {
                b = 0;
            }
            // println!(
            //     "connecting {} --> {}",
            //     points.get(a).unwrap().display(),
            //     points.get(b).unwrap().display()
            // );
            grid.connect_red_tiles_with_green_tiles(
                points.get(a).unwrap(),
                points.get(b).unwrap(),
            );
            a = b;
        }
        println!(
            "connecting red tiles took {} secs",
            now.elapsed().as_secs_f64()
        );
        println!("\nOUTLINED:");
        if grid.max_x < 50 && grid.max_y < 50 {
            grid.display_grid();
        }

        let now = Instant::now();
        let mut a = 0;
        for next in 1..=len {
            let mut b = next;
            if next == len {
                b = 0;
            }
            let p_a = points.get(a).unwrap();
            let p_b = points.get(b).unwrap();
            // println!(
            //     "marking {} ({:?})--> {}",
            //     points.get(a).unwrap().display(),
            //     grid.get_inside_direction(p_a.x, p_a.y),
            //     points.get(b).unwrap().display()
            // );
            grid.mark_red_tiles_with_inside_direction(
                points.get(a).unwrap(),
                points.get(b).unwrap(),
            );
            // println!(
            //     "a is now {:?}; b is now {:?}",
            //     grid.get_inside_direction(p_a.x, p_a.y),
            //     grid.get_inside_direction(p_b.x, p_b.y)
            // );
            a = b;
        }
        println!(
            "marking inside orientation of red tiles took {} secs",
            now.elapsed().as_secs_f64()
        );

        let now = Instant::now();
        grid.fill_in_loops();
        // println!("\nFILLED:");
        // grid.display_grid();
        println!(
            "filling loops took {} secs",
            now.elapsed().as_secs_f64()
        );

        let now = Instant::now();
        let mut max_area: u64 = 0;
        let len = points.len();
        grid.find_max_filled_area(&mut max_area, &points, 0..len);
        println!(
            "find_max_filled_area() took {} secs",
            now.elapsed().as_secs_f64()
        );

        println!("Max area: {}", max_area);
    }

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
    println!("\nOUTLINED:");
    grid.display_grid();

    let mut a = 0;
    for next in 1..=len {
        let mut b = next;
        if next == len {
            b = 0;
        }
        grid.mark_red_tiles_with_inside_direction(
            points.get(a).unwrap(),
            points.get(b).unwrap(),
        );
        a = b;
    }
    println!("\nMARKED:");

    grid.fill_in_loops();
    println!("\nFILLED:");
    grid.display_grid();

    let mut max_area: u64 = 0;
    let len = points.len();
    grid.find_max_filled_area(&mut max_area, &points, 0..len);

    assert_eq!(24, max_area);
}

#[test]
fn t_degen_example_part_2() {
    let raw_input = "3,1
6,1
6,3
11,3
11,1
15,1
15,5
9,5
9,6
6,6
6,8
1,8
1,5
3,5"
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
    println!("\nOUTLINED:");
    grid.display_grid();

    let mut a = 0;
    for next in 1..=len {
        let mut b = next;
        if next == len {
            b = 0;
        }
        grid.mark_red_tiles_with_inside_direction(
            points.get(a).unwrap(),
            points.get(b).unwrap(),
        );
        a = b;
    }
    println!("\nMARKED:");

    grid.fill_in_loops();
    println!("\nFILLED:");
    grid.display_grid();

    let mut max_area: u64 = 0;
    let len = points.len();
    grid.find_max_filled_area(&mut max_area, &points, 0..len);

    assert_eq!(32, max_area);
}
