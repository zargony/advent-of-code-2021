use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Movement parse error
#[derive(Debug, Error)]
#[error("Bad movement")]
struct BadMovement;

/// Movement direction and distance
#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Movement {
    type Err = BadMovement;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(' ').ok_or(BadMovement)?;
        let distance: u32 = distance.parse().map_err(|_| BadMovement)?;
        match direction {
            "forward" => Ok(Movement::Forward(distance)),
            "down" => Ok(Movement::Down(distance)),
            "up" => Ok(Movement::Up(distance)),
            _ => Err(BadMovement),
        }
    }
}

/// Submarine position
#[derive(Debug, Default, Clone)]
struct Position {
    position: u32,
    depth: u32,
}

impl Position {
    /// Move along the given course
    fn go(&mut self, course: &[Movement]) {
        for movement in course {
            match movement {
                Movement::Forward(distance) => self.position += distance,
                Movement::Down(distance) => self.depth += distance,
                Movement::Up(distance) => self.depth -= distance,
            }
        }
    }
}

/// Submarine position (part 2)
#[derive(Debug, Default, Clone)]
struct ExactPosition {
    position: u32,
    depth: u32,
    aim: u32,
}

impl ExactPosition {
    /// Move along the given course
    fn go(&mut self, course: &[Movement]) {
        for movement in course {
            match movement {
                Movement::Forward(distance) => {
                    self.position += distance;
                    self.depth += self.aim * distance;
                }
                Movement::Down(distance) => self.aim += distance,
                Movement::Up(distance) => self.aim -= distance,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let course: Vec<Movement> = Input::day(2)?.parsed_lines().try_collect()?;

    let mut position = Position::default();
    position.go(&course);
    println!(
        "Final position: {}, depth: {}, product: {}",
        position.position,
        position.depth,
        position.position * position.depth,
    );

    let mut position = ExactPosition::default();
    position.go(&course);
    println!(
        "Final exact position: {}, depth: {}, product: {}",
        position.position,
        position.depth,
        position.position * position.depth,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const COURSE: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    fn course() -> Vec<Movement> {
        COURSE.iter().map(|s| s.parse().unwrap()).collect()
    }

    #[test]
    fn parse() {
        assert_eq!(
            course(),
            [
                Movement::Forward(5),
                Movement::Down(5),
                Movement::Forward(8),
                Movement::Up(3),
                Movement::Down(8),
                Movement::Forward(2),
            ]
        );
    }

    #[test]
    fn part_1() {
        let mut position = Position::default();
        position.go(&course());
        assert_eq!(position.position, 15);
        assert_eq!(position.depth, 10);
    }

    #[test]
    fn part_2() {
        let mut position = ExactPosition::default();
        position.go(&course());
        assert_eq!(position.position, 15);
        assert_eq!(position.depth, 60);
    }
}
