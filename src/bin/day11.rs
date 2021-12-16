use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Grid of dumb octopuses
#[derive(Debug)]
struct Grid(Vec<Vec<u8>>);

impl<S: AsRef<str>> TryFrom<&[S]> for Grid {
    type Error = ParseError;

    fn try_from(heightmap: &[S]) -> Result<Self, Self::Error> {
        Ok(Self(
            heightmap
                .iter()
                .map(|line| {
                    line.as_ref()
                        .chars()
                        .map(|ch| {
                            ch.to_digit(10)
                                .ok_or(ParseError)
                                .and_then(|n| u8::try_from(n).map_err(|_| ParseError))
                        })
                        .try_collect()
                })
                .try_collect()?,
        ))
    }
}

impl Grid {
    /// Increase energy level of given cell
    fn increase(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.0.get_mut(y).and_then(|row| row.get_mut(x)) {
            *cell += 1;
            // If cell was just triggered to flash, increase adjacent cells as well
            if *cell == 10 {
                if x > 0 && y > 0 {
                    self.increase(x - 1, y - 1);
                }
                if x > 0 {
                    self.increase(x - 1, y);
                    self.increase(x - 1, y + 1);
                }
                if y > 0 {
                    self.increase(x, y - 1);
                    self.increase(x + 1, y - 1);
                }
                self.increase(x, y + 1);
                self.increase(x + 1, y);
                self.increase(x + 1, y + 1);
            }
        }
    }

    /// Do one step, return number of flashes
    fn step(&mut self) -> usize {
        // Increase energy of all cells
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                self.increase(x, y);
            }
        }
        // Flash all overloaded cells
        let mut flashes = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if let Some(cell) = self.0.get_mut(y).and_then(|row| row.get_mut(x)) {
                    if *cell >= 10 {
                        *cell = 0;
                        flashes += 1;
                    }
                }
            }
        }
        flashes
    }

    /// Do one step, return number of flashes
    fn steps(&mut self, count: usize) -> usize {
        (0..count).map(|_| self.step()).sum()
    }

    /// Step until all octopuses flash, return number of steps
    fn step_until_full_flash(&mut self) -> usize {
        let mut steps = 0;
        loop {
            steps += 1;
            if self.step() == 100 {
                return steps;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<_> = Input::day(11)?.lines().try_collect()?;

    let mut grid = Grid::try_from(&lines[..])?;
    let flashes = grid.steps(100);
    println!("Total flashes after 100 steps: {}", flashes);

    let mut grid = Grid::try_from(&lines[..])?;
    let steps = grid.step_until_full_flash();
    println!("Steps until full flash: {}", steps);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [&str; 10] = [
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    fn grid() -> Grid {
        Grid::try_from(&GRID[..]).unwrap()
    }

    #[test]
    fn part_1a() {
        let mut grid = grid();
        assert_eq!(grid.step(), 0);
        assert_eq!(grid.step(), 35);
        assert_eq!(grid.step(), 45);
        assert_eq!(grid.step(), 16);
        assert_eq!(grid.step(), 8);
        assert_eq!(grid.step(), 1);
        assert_eq!(grid.step(), 7);
        assert_eq!(grid.step(), 24);
        assert_eq!(grid.step(), 39);
        assert_eq!(grid.step(), 29);
        grid.steps(9);
        assert_eq!(grid.step(), 28);
        grid.steps(9);
        assert_eq!(grid.step(), 1);
        grid.steps(9);
        assert_eq!(grid.step(), 12);
        grid.steps(9);
        assert_eq!(grid.step(), 27);
        grid.steps(9);
        assert_eq!(grid.step(), 3);
        grid.steps(9);
        assert_eq!(grid.step(), 13);
        grid.steps(9);
        assert_eq!(grid.step(), 40);
        grid.steps(9);
        assert_eq!(grid.step(), 0);
        grid.steps(9);
        assert_eq!(grid.step(), 13);
    }

    #[test]
    fn part_1b() {
        let mut grid = grid();
        assert_eq!(grid.steps(100), 1656);
    }

    #[test]
    fn part_2() {
        let mut grid = grid();
        assert_eq!(grid.step_until_full_flash(), 195);
    }
}
