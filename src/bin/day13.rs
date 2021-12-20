use advent_of_code_2021::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use std::{error, fmt};
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Fold instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, pos) = s.split_once('=').ok_or(ParseError)?;
        let pos = pos.parse().map_err(|_| ParseError)?;
        match prefix {
            "fold along y" => Ok(Self::Horizontal(pos)),
            "fold along x" => Ok(Self::Vertical(pos)),
            _ => Err(ParseError),
        }
    }
}

/// Piece of transparent paper
#[derive(Debug)]
struct Paper {
    dots: HashSet<(usize, usize)>,
}

impl<S: AsRef<str>> TryFrom<&[S]> for Paper {
    type Error = ParseError;

    fn try_from(lines: &[S]) -> Result<Self, Self::Error> {
        let mut dots = HashSet::new();
        for line in lines {
            let (x, y) = line.as_ref().split_once(',').ok_or(ParseError)?;
            let x: usize = x.parse().map_err(|_| ParseError)?;
            let y: usize = y.parse().map_err(|_| ParseError)?;
            dots.insert((x, y));
        }
        Ok(Self { dots })
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.dimension();
        for y in 0..height {
            for x in 0..width {
                match self.dots.contains(&(x, y)) {
                    false => write!(f, ".")?,
                    true => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Paper {
    /// Count number of dots
    fn count(&self) -> usize {
        self.dots.len()
    }

    /// Width and height of paper
    fn dimension(&self) -> (usize, usize) {
        self.dots
            .iter()
            .fold((0, 0), |(w, h), (x, y)| (w.max(*x + 1), h.max(*y + 1)))
    }

    /// Fold paper
    fn fold(&mut self, fold: &Fold) {
        self.dots = self
            .dots
            .drain()
            .map(|coord| match fold {
                Fold::Horizontal(y) if coord.1 > *y => (coord.0, y - (coord.1 - y)),
                Fold::Vertical(x) if coord.0 > *x => (x - (coord.0 - x), coord.1),
                _ => coord,
            })
            .collect();
    }

    /// Fold paper many times
    fn fold_many(&mut self, folds: &[Fold]) {
        for fold in folds {
            self.fold(fold)
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut blocks = Input::day(13)?.blocks();
    let lines = blocks.next().ok_or(ParseError)??;
    let mut paper = Paper::try_from(&lines[..])?;
    let lines = blocks.next().ok_or(ParseError)??;
    let folds: Vec<Fold> = lines.iter().map(|line| line.parse()).try_collect()?;

    paper.fold(&folds[0]);
    println!("Number of dots after 1st fold: {}", paper.count());

    paper.fold_many(&folds[1..]);
    println!("Resulting folded paper:\n{}", paper);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOTS: [&str; 18] = [
        "6,10", "0,14", "9,10", "0,3", "10,4", "4,11", "6,0", "6,12", "4,1", "0,13", "10,12",
        "3,4", "3,0", "8,4", "1,10", "2,14", "8,10", "9,0",
    ];
    const FOLDS: [&str; 2] = ["fold along y=7", "fold along x=5"];

    fn paper() -> Paper {
        Paper::try_from(&DOTS[..]).unwrap()
    }

    fn folds() -> [Fold; 2] {
        FOLDS.map(|s| s.parse().unwrap())
    }

    #[test]
    fn part_1() {
        let (mut paper, folds) = (paper(), folds());
        assert_eq!(paper.count(), 18);
        paper.fold(&folds[0]);
        assert_eq!(paper.count(), 17);
        paper.fold(&folds[1]);
        assert_eq!(paper.count(), 16);
    }
}
