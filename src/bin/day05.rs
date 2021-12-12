use advent_of_code_2021::Input;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error;
use std::ops::{Add, Mul};
use std::str::FromStr;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError).and_then(|(sx, sy)| {
            Ok((
                sx.trim().parse().map_err(|_| ParseError)?,
                sy.trim().parse().map_err(|_| ParseError)?,
            ))
        })?;
        Ok(Self::new(x, y))
    }
}

impl Add<Offset> for Coordinate {
    type Output = Self;

    fn add(self, offset: Offset) -> Self {
        Self {
            x: (self.x as isize + offset.x) as usize,
            y: (self.y as isize + offset.y) as usize,
        }
    }
}

impl Coordinate {
    /// Create a new coordinate with given x and y position
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Coordinate offset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset {
    x: isize,
    y: isize,
}

impl Mul<isize> for Offset {
    type Output = Self;

    fn mul(self, factor: isize) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Offset {
    /// Create a new offset with given x and y values
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

/// Line of vents
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    from: Coordinate,
    to: Coordinate,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once("->")
            .ok_or(ParseError)
            .and_then(|(sfrom, sto)| {
                Ok((
                    sfrom.parse().map_err(|_| ParseError)?,
                    sto.parse().map_err(|_| ParseError)?,
                ))
            })?;
        Ok(Self::new(from, to))
    }
}

impl Line {
    /// Create a new line with given coordinates
    fn new(from: Coordinate, to: Coordinate) -> Self {
        Self { from, to }
    }

    /// Direction of line
    fn direction(&self) -> Offset {
        let ofsx = match self.from.x.cmp(&self.to.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let ofsy = match self.from.y.cmp(&self.to.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        Offset::new(ofsx, ofsy)
    }

    /// Is diagonal line
    fn is_diagonal(&self) -> bool {
        let direction = self.direction();
        direction.x != 0 && direction.y != 0
    }

    /// Return a list of coordinates the line goes through
    fn coordinates(&self) -> Vec<Coordinate> {
        let minx = usize::min(self.from.x, self.to.x);
        let maxx = usize::max(self.from.x, self.to.x);
        let lenx = maxx - minx;
        let miny = usize::min(self.from.y, self.to.y);
        let maxy = usize::max(self.from.y, self.to.y);
        let leny = maxy - miny;
        if self.from.x == self.to.x {
            (miny..=maxy)
                .map(|y| Coordinate::new(self.from.x, y))
                .collect()
        } else if self.from.y == self.to.y {
            (minx..=maxx)
                .map(|x| Coordinate::new(x, self.from.y))
                .collect()
        } else if lenx == leny {
            let direction = self.direction();
            (0..=lenx as isize)
                .map(|i| self.from + direction * i)
                .collect()
        } else {
            panic!("Only horizontal, vertical and diagonal lines are supported");
        }
    }
}

/// Ocean floow
#[derive(Debug)]
struct Floor {
    density: HashMap<Coordinate, usize>,
    ignore_diagonals: bool,
}

impl From<(bool, &[Line])> for Floor {
    fn from(input: (bool, &[Line])) -> Self {
        let (ignore_diagonals, lines) = input;
        let mut floor = Self::new(ignore_diagonals);
        for line in lines {
            floor.add_line(line);
        }
        floor
    }
}

impl Floor {
    /// Create a new, empty ocean floor
    fn new(ignore_diagonals: bool) -> Self {
        Self {
            density: HashMap::new(),
            ignore_diagonals,
        }
    }

    /// Add a line of vents to the ocean floor
    fn add_line(&mut self, line: &Line) {
        if !(self.ignore_diagonals && line.is_diagonal()) {
            for coord in line.coordinates() {
                self.density
                    .entry(coord)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    /// Find number of danger areas (where density is >= 2)
    fn num_danger_areas(&self) -> usize {
        self.density.values().filter(|d| **d >= 2).count()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<Line> = Input::day(5)?.parsed_lines().try_collect()?;

    let floor = Floor::from((true, &lines[..]));
    println!("Number of danger areas: {}", floor.num_danger_areas());

    let floor = Floor::from((false, &lines[..]));
    println!(
        "Number of danger areas with diagonals: {}",
        floor.num_danger_areas()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    fn lines() -> [Line; 10] {
        LINES.map(|line| line.parse().unwrap())
    }

    #[test]
    fn parse() {
        assert_eq!(
            lines(),
            [
                Line::new(Coordinate::new(0, 9), Coordinate::new(5, 9)),
                Line::new(Coordinate::new(8, 0), Coordinate::new(0, 8)),
                Line::new(Coordinate::new(9, 4), Coordinate::new(3, 4)),
                Line::new(Coordinate::new(2, 2), Coordinate::new(2, 1)),
                Line::new(Coordinate::new(7, 0), Coordinate::new(7, 4)),
                Line::new(Coordinate::new(6, 4), Coordinate::new(2, 0)),
                Line::new(Coordinate::new(0, 9), Coordinate::new(2, 9)),
                Line::new(Coordinate::new(3, 4), Coordinate::new(1, 4)),
                Line::new(Coordinate::new(0, 0), Coordinate::new(8, 8)),
                Line::new(Coordinate::new(5, 5), Coordinate::new(8, 2)),
            ]
        );
    }

    #[test]
    fn part_1() {
        let floor = Floor::from((true, &lines()[..]));
        assert_eq!(floor.num_danger_areas(), 5);
    }

    #[test]
    fn part_2() {
        let floor = Floor::from((false, &lines()[..]));
        assert_eq!(floor.num_danger_areas(), 12);
    }
}
