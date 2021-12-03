use advent_of_code_2021::Input;
use std::error;
use std::fmt::Display;
use std::num::ParseIntError;

/// Distribution of bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Distribution {
    MostCommonZero,
    MostCommonOne,
    EquallyCommon,
}

/// Diagnostic report
#[derive(Debug, Clone)]
struct Diag(Vec<u16>, usize);

impl Diag {
    /// Create new dignostic report
    fn new<I, L>(lines: I) -> Result<Self, ParseIntError>
    where
        I: IntoIterator<Item = L>,
        L: AsRef<str> + Display,
    {
        let mut len = 0;
        let numbers = lines
            .into_iter()
            .map(|line| {
                len = len.max(line.as_ref().len());
                u16::from_str_radix(line.as_ref(), 2)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(numbers, len))
    }

    /// Count one bits at position i
    fn count_ones(&self, i: usize) -> usize {
        self.0.iter().filter(|n| *n & (1 << i) > 0).count()
    }

    /// Distribution of bits at position i
    fn distribution(&self, i: usize) -> Distribution {
        let ones = self.count_ones(i);
        if ones * 2 == self.0.len() {
            Distribution::EquallyCommon
        } else if ones > self.0.len() / 2 {
            Distribution::MostCommonOne
        } else {
            Distribution::MostCommonZero
        }
    }

    /// Gamma rate
    fn gamma(&self) -> usize {
        (0..self.1).fold(0, |gamma, i| match self.distribution(i) {
            Distribution::MostCommonOne => gamma | 1 << i,
            _ => gamma,
        })
    }

    /// Epsilon rate
    fn epsilon(&self) -> usize {
        (0..self.1).fold(0, |epsilon, i| match self.distribution(i) {
            Distribution::MostCommonOne => epsilon,
            _ => epsilon | 1 << i,
        })
    }

    /// Power consumption
    fn power(&self) -> usize {
        self.gamma() * self.epsilon()
    }

    /// Filter entries with given bit in position i
    fn filter(&mut self, i: usize, bit: bool) {
        self.0.retain(|n| (*n & (1 << i) > 0) == bit);
    }

    /// Oxygen generator rating
    fn oxygen(&self) -> u16 {
        let mut diag = self.clone();
        for i in (0..self.1).rev() {
            if diag.0.len() < 2 {
                break;
            }
            diag.filter(i, diag.distribution(i) != Distribution::MostCommonZero);
        }
        diag.0[0]
    }

    /// CO2 scrubber rating
    fn co2(&self) -> u16 {
        let mut diag = self.clone();
        for i in (0..self.1).rev() {
            if diag.0.len() < 2 {
                break;
            }
            diag.filter(i, diag.distribution(i) == Distribution::MostCommonZero);
        }
        diag.0[0]
    }

    /// Life support rating
    fn life_support(&self) -> usize {
        self.oxygen() as usize * self.co2() as usize
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let diag = Diag::new(&Input::day(3)?.lines()?)?;

    println!(
        "Gamma: {}, epsilon: {}, power: {}",
        diag.gamma(),
        diag.epsilon(),
        diag.power()
    );

    println!(
        "Oxygen: {}, CO2: {}, life support: {}",
        diag.oxygen(),
        diag.co2(),
        diag.life_support()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIAG: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn part_1() {
        let diag = Diag::new(&DIAG).unwrap();
        assert_eq!(diag.gamma(), 22);
        assert_eq!(diag.epsilon(), 9);
    }

    #[test]
    fn part_2() {
        let diag = Diag::new(&DIAG).unwrap();
        assert_eq!(diag.oxygen(), 23);
        assert_eq!(diag.co2(), 10);
    }
}
