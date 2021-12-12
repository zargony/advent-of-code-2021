use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Lanternfish population
#[derive(Debug)]
struct Population {
    /// Population count grouped by state. I.e. statecount[5] has the
    /// number of lanternfish with a state of 5
    statecount: [usize; 9],
}

impl TryFrom<&[u8]> for Population {
    type Error = ParseError;

    fn try_from(states: &[u8]) -> Result<Self, Self::Error> {
        let mut statecount = [0; 9];
        for state in states {
            if (0..9).contains(state) {
                statecount[*state as usize] += 1;
            } else {
                return Err(ParseError);
            }
        }
        Ok(Self { statecount })
    }
}

impl FromStr for Population {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let states: Vec<u8> = s
            .split(',')
            .map(|s| s.parse())
            .try_collect()
            .map_err(|_| ParseError)?;
        Self::try_from(&states[..])
    }
}

impl Population {
    /// Evolve next day
    fn evolve(&mut self, days: usize) {
        for _ in 0..days {
            // Each day, all lanternfish decrease their state by 1. Lanternfish
            // that reached 0, start at 6 again and generate an offspring that
            // starts at 8
            self.statecount = [
                self.statecount[1],
                self.statecount[2],
                self.statecount[3],
                self.statecount[4],
                self.statecount[5],
                self.statecount[6],
                self.statecount[0] + self.statecount[7],
                self.statecount[8],
                self.statecount[0], // offspring
            ];
        }
    }

    /// Total number of lanternfish
    fn count(&self) -> usize {
        self.statecount.iter().sum()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let line = Input::day(6)?.line()?;
    let mut population: Population = line.parse()?;

    population.evolve(80);
    println!("Population after 80 days: {}", population.count());

    population.evolve(256 - 80);
    println!("Population after 256 days: {}", population.count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_STATE: [u8; 5] = [3, 4, 3, 1, 2];

    fn population() -> Population {
        Population::try_from(&INITIAL_STATE[..]).unwrap()
    }

    #[test]
    fn part_1_and_2() {
        let mut population = population();
        population.evolve(18);
        assert_eq!(population.count(), 26);
        population.evolve(80 - 18);
        assert_eq!(population.count(), 5934);
        population.evolve(256 - 80);
        assert_eq!(population.count(), 26984457539);
    }
}
