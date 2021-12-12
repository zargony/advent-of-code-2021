use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Fuel calculation model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FuelModel {
    /// Simple fuel model: 1 fuel per 1 distance
    Simple,
    /// Realistic fuel model: fuel increases per distance (1+2+3+4 for distance 4)
    Realistic,
}

impl FuelModel {
    /// Calculate fuel for the given distance
    fn fuel_for_distance(&self, distance: usize) -> usize {
        match self {
            FuelModel::Simple => distance,
            FuelModel::Realistic => (1 + distance) * distance / 2,
        }
    }
}

/// Swarm of crabs
#[derive(Debug)]
struct Swarm {
    positions: Vec<usize>,
}

impl From<&[usize]> for Swarm {
    fn from(positions: &[usize]) -> Self {
        Self {
            positions: positions.into(),
        }
    }
}

impl FromStr for Swarm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<usize> = s
            .split(',')
            .map(|s| s.parse())
            .try_collect()
            .map_err(|_| ParseError)?;
        Ok(Self::from(&positions[..]))
    }
}

impl Swarm {
    /// Find max (rightmost) position
    fn max_position(&self) -> usize {
        self.positions.iter().copied().max().unwrap_or(0)
    }

    /// Calculate fuel for moving everyone to the given position
    fn fuel_required(&self, position: usize, model: FuelModel) -> usize {
        self.positions
            .iter()
            .copied()
            .map(|pos| {
                let distance = if pos > position {
                    pos - position
                } else {
                    position - pos
                };
                model.fuel_for_distance(distance)
            })
            .sum()
    }

    /// Calculate position with least fuel requirement
    fn least_fuel_required(&self, model: FuelModel) -> (usize, usize) {
        // Brute-force find the least fuel requirement
        (0..self.max_position())
            .map(|pos| (pos, self.fuel_required(pos, model)))
            .min_by_key(|(_pos, fuel)| *fuel)
            .unwrap_or((0, 0))
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let line = Input::day(7)?.line()?;
    let swarm: Swarm = line.parse()?;

    let (position, fuel) = swarm.least_fuel_required(FuelModel::Simple);
    println!("Aligning at {} uses least fuel: {}", position, fuel);

    let (position, fuel) = swarm.least_fuel_required(FuelModel::Realistic);
    println!(
        "Realistic aligning at {} uses least fuel: {}",
        position, fuel
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const HORIZONTAL_POSITIONS: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    fn swarm() -> Swarm {
        Swarm::from(&HORIZONTAL_POSITIONS[..])
    }

    #[test]
    fn fuel_realistic() {
        assert_eq!(FuelModel::Realistic.fuel_for_distance(1), 1);
        assert_eq!(FuelModel::Realistic.fuel_for_distance(2), 3);
        assert_eq!(FuelModel::Realistic.fuel_for_distance(3), 6);
        assert_eq!(FuelModel::Realistic.fuel_for_distance(4), 10);
        assert_eq!(FuelModel::Realistic.fuel_for_distance(5), 15);
    }

    #[test]
    fn part_1() {
        let swarm = swarm();
        assert_eq!(swarm.fuel_required(1, FuelModel::Simple), 41);
        assert_eq!(swarm.fuel_required(2, FuelModel::Simple), 37);
        assert_eq!(swarm.fuel_required(3, FuelModel::Simple), 39);
        assert_eq!(swarm.fuel_required(10, FuelModel::Simple), 71);
        assert_eq!(swarm.least_fuel_required(FuelModel::Simple), (2, 37));
    }

    #[test]
    fn part_2() {
        let swarm = swarm();
        assert_eq!(swarm.fuel_required(2, FuelModel::Realistic), 206);
        assert_eq!(swarm.fuel_required(5, FuelModel::Realistic), 168);
        assert_eq!(swarm.least_fuel_required(FuelModel::Realistic), (5, 168));
    }
}
