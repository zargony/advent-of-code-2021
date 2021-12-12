use advent_of_code_2021::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// A segment of a 7-segment digit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = ParseError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            _ => Err(ParseError),
        }
    }
}

/// A 7-segment digit
#[derive(Debug, Clone, PartialEq, Eq)]
struct Digit(HashSet<Segment>);

impl FromStr for Digit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().chars().map(Segment::try_from).try_collect()?))
    }
}

impl Digit {
    /// Detect `1`: it's unique by having exactly 2 segments active
    fn is_one(&self) -> bool {
        self.0.len() == 2
    }

    /// Detect `4`: it's unique by having exactly 4 segments active
    fn is_four(&self) -> bool {
        self.0.len() == 4
    }

    /// Detect simple numbers `1`, `4`, `7` and `8`: they're unique by
    /// having exactly 2, 4, 3 and 7 segments active.
    fn is_simple_number(&self) -> bool {
        [2, 4, 3, 7].contains(&self.0.len())
    }

    /// Determine number of segments that overlap with the given other digit
    fn overlap(&self, other: &Self) -> usize {
        (&self.0 & &other.0).len()
    }

    /// Determine which number this digit represents. To determine non-simple
    /// numbers, simple number digits `1` and `4` must be given as reference
    fn number(&self, one: &Digit, four: &Digit) -> Option<u8> {
        match (self.0.len(), self.overlap(one), self.overlap(four)) {
            // 2 active segments must be `1`
            (2, _, _) => Some(1),
            // 3 active segments must be `7`
            (3, _, _) => Some(7),
            // 4 active segments must be `4`
            (4, _, _) => Some(4),
            // 5 active segments can be `2`, `3` or `5`
            // 5 active segments with 1 same as `1` and 2 same as `4` must be `2`
            (5, 1, 2) => Some(2),
            // 5 active segments with 1 same as `1` and 3 same as `4` must be `5`
            (5, 1, 3) => Some(5),
            // 5 active segments with 2 same as `1` must be `3`
            (5, 2, _) => Some(3),
            // 6 active segments can be `0`, `6` or `9`
            // 6 active segments with 1 same as `1` must be `6`
            (6, 1, _) => Some(6),
            // 6 active segments with 2 same as `1` and 3 same as `4` must be `0`
            (6, 2, 3) => Some(0),
            // 6 active segments with 2 same as `1` and 4 same as `4` must be `9`
            (6, 2, 4) => Some(9),
            // 7 active segments must be `8`
            (7, _, _) => Some(8),
            // Otherwise can't determine
            _ => None,
        }
    }
}

/// An entry of observed digits
#[derive(Debug)]
struct Entry {
    patterns: [Digit; 10],
    digits: [Digit; 4],
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once('|').ok_or(ParseError)?;
        let patterns: Vec<Digit> = s1.split_whitespace().map(|s| s.parse()).try_collect()?;
        let digits: Vec<Digit> = s2.split_whitespace().map(|s| s.parse()).try_collect()?;
        Ok(Self {
            patterns: patterns.try_into().map_err(|_| ParseError)?,
            digits: digits.try_into().map_err(|_| ParseError)?,
        })
    }
}

impl Entry {
    /// Find `1` digit in patterns
    fn one(&self) -> Option<&Digit> {
        self.patterns.iter().find(|digit| digit.is_one())
    }

    /// Find `4` digit in patterns
    fn four(&self) -> Option<&Digit> {
        self.patterns.iter().find(|digit| digit.is_four())
    }

    /// Returns number of `1`, `4`, `7` and `8` digits
    fn count_simple_number_digits(&self) -> usize {
        self.digits
            .iter()
            .map(|digit| if digit.is_simple_number() { 1 } else { 0 })
            .sum()
    }

    /// Determine value of digits
    fn value(&self) -> Option<usize> {
        let one = self.one()?;
        let four = self.four()?;
        Some(
            self.digits[0].number(one, four)? as usize * 1000
                + self.digits[1].number(one, four)? as usize * 100
                + self.digits[2].number(one, four)? as usize * 10
                + self.digits[3].number(one, four)? as usize,
        )
    }
}

fn count_simple_number_digits(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|entry| entry.count_simple_number_digits())
        .sum()
}

fn sum_of_values(entries: &[Entry]) -> Option<usize> {
    entries
        .iter()
        .fold(Some(0), |sum, entry| match (sum, entry.value()) {
            (Some(sum), Some(value)) => Some(sum + value),
            _ => None,
        })
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let entries: Vec<Entry> = Input::day(8)?.parsed_lines().try_collect()?;

    println!("Simple digits: {}", count_simple_number_digits(&entries));

    println!("Sum of values: {}", sum_of_values(&entries).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const ENTRIES: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    fn entry() -> Entry {
        ENTRY.parse().unwrap()
    }

    fn entries() -> [Entry; 10] {
        ENTRIES.map(|line| line.parse().unwrap())
    }

    #[test]
    fn part_1() {
        let entry = entry();
        assert!(entry.patterns[0].is_simple_number());
        assert!(entry.patterns[4].is_simple_number());
        assert!(entry.patterns[7].is_four());
        assert!(entry.patterns[7].is_simple_number());
        assert!(entry.patterns[9].is_one());
        assert!(entry.patterns[9].is_simple_number());
        assert_eq!(entry.count_simple_number_digits(), 0);

        let entries = entries();
        assert_eq!(count_simple_number_digits(&entries), 26);
    }

    #[test]
    fn part_2() {
        let entry = entry();
        assert_eq!(entry.value(), Some(5353));

        let entries = entries();
        assert_eq!(entries[0].value(), Some(8394));
        assert_eq!(entries[1].value(), Some(9781));
        assert_eq!(entries[2].value(), Some(1197));
        assert_eq!(entries[3].value(), Some(9361));
        assert_eq!(entries[4].value(), Some(4873));
        assert_eq!(entries[5].value(), Some(8418));
        assert_eq!(entries[6].value(), Some(4548));
        assert_eq!(entries[7].value(), Some(1625));
        assert_eq!(entries[8].value(), Some(8717));
        assert_eq!(entries[9].value(), Some(4315));
        assert_eq!(sum_of_values(&entries), Some(61229));
    }
}
