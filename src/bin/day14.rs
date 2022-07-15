use advent_of_code_2021::Input;
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Polymer pair insertion rule set
#[derive(Debug)]
struct Rules(HashMap<(char, char), char>);

impl<S: AsRef<str>> TryFrom<&[S]> for Rules {
    type Error = ParseError;

    fn try_from(lines: &[S]) -> Result<Self, Self::Error> {
        let mut rules = HashMap::new();
        for line in lines {
            let (left, right) = line.as_ref().split_once("->").ok_or(ParseError)?;
            let mut left = left.trim().chars();
            let first = left.next().ok_or(ParseError)?;
            let second = left.next().ok_or(ParseError)?;
            left.next().is_none().then(|| ()).ok_or(ParseError)?;
            let mut right = right.trim().chars();
            let insert = right.next().ok_or(ParseError)?;
            right.next().is_none().then(|| ()).ok_or(ParseError)?;
            rules.insert((first, second), insert);
        }
        Ok(Self(rules))
    }
}

impl Rules {
    /// Get insertion character for given sequence
    fn get(&self, a: char, b: char) -> Option<char> {
        self.0.get(&(a, b)).copied()
    }
}

/// Polymer
///
/// For performance and scaling reasons, this doesn't keep the whole polymer
/// string but only a count of unique element groups
#[derive(Debug, Clone, PartialEq, Eq)]
struct Polymer {
    groups: HashMap<(char, char), usize>,
    last: (char, char),
}

impl FromStr for Polymer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut groups, (a, b)| {
                groups.entry((a, b)).and_modify(|e| *e += 1).or_insert(1);
                groups
            });
        let last = s.chars().tuple_windows().last().ok_or(ParseError)?;
        Ok(Self { groups, last })
    }
}

impl Polymer {
    /// Calculate actual length of polymer
    #[cfg(test)]
    fn len(&self) -> usize {
        self.groups.values().sum::<usize>() + 1
    }

    /// Appply one step of the given rules
    fn step(&mut self, rules: &Rules) {
        self.groups = self
            .groups
            .iter()
            .flat_map(|((a, b), n)| match rules.get(*a, *b) {
                Some(insert) => vec![(*a, insert, *n), (insert, *b, *n)],
                None => vec![(*a, *b, *n)],
            })
            .fold(HashMap::new(), |mut groups, (a, b, n)| {
                groups.entry((a, b)).and_modify(|e| *e += n).or_insert(n);
                groups
            });
        if let Some(insert) = rules.get(self.last.0, self.last.1) {
            self.last.0 = insert;
        }
    }

    /// Apply multiple steps using the given rules
    fn process(&mut self, steps: usize, rules: &Rules) {
        for _ in 0..steps {
            self.step(rules);
        }
    }

    /// Counts of polymer elements
    fn counts(&self) -> HashMap<char, usize> {
        self.groups
            .iter()
            .fold([(self.last.1, 1)].into(), |mut counts, ((a, _b), n)| {
                counts.entry(*a).and_modify(|e| *e += *n).or_insert(*n);
                counts
            })
    }

    /// Calculate most-least-score
    fn most_least_score(&self) -> usize {
        match self.counts().values().minmax() {
            MinMaxResult::NoElements => 0,
            MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => *max - *min,
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = Input::day(14)?;
    let mut polymer: Polymer = input.line()?.parse()?;
    input.line()?;
    let lines: Vec<_> = input.lines().try_collect()?;
    let rules = Rules::try_from(&lines[..])?;

    polymer.process(10, &rules);
    println!(
        "Most/least common element score (10 steps): {}",
        polymer.most_least_score()
    );

    polymer.process(30, &rules);
    println!(
        "Most/least common element score (40 steps): {}",
        polymer.most_least_score()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: [&str; 16] = [
        "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
        "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
    ];

    fn polymer() -> Polymer {
        "NNCB".parse().unwrap()
    }

    fn rules() -> Rules {
        Rules::try_from(&RULES[..]).unwrap()
    }

    #[test]
    fn part_1() {
        let rules = rules();

        let mut polymer = polymer();
        assert_eq!(polymer.len(), 4); // NNCB
        assert_eq!(polymer.counts(), [('B', 1), ('C', 1), ('N', 2)].into());

        polymer.process(1, &rules);
        assert_eq!(polymer.len(), 7); // NCNBCHB
        assert_eq!(
            polymer.counts(),
            [('B', 2), ('C', 2), ('H', 1), ('N', 2)].into()
        );

        polymer.process(1, &rules);
        assert_eq!(polymer.len(), 13); // NBCCNBBBCBHCB
        assert_eq!(
            polymer.counts(),
            [('B', 6), ('C', 4), ('H', 1), ('N', 2)].into()
        );

        polymer.process(1, &rules);
        assert_eq!(polymer.len(), 25); // NBBBCNCCNBBNBNBBCHBHHBCHB
        assert_eq!(
            polymer.counts(),
            [('B', 11), ('C', 5), ('H', 4), ('N', 5)].into()
        );

        polymer.process(1, &rules);
        assert_eq!(polymer.len(), 49); // NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
        assert_eq!(
            polymer.counts(),
            [('B', 23), ('C', 10), ('H', 5), ('N', 11)].into()
        );

        polymer.process(1, &rules);
        assert_eq!(polymer.len(), 97);

        polymer.process(5, &rules);
        assert_eq!(polymer.len(), 3073);
        assert_eq!(polymer.most_least_score(), 1588);
    }

    #[test]
    fn part_2() {
        let rules = rules();

        let mut polymer = polymer();
        polymer.process(40, &rules);
        assert_eq!(polymer.most_least_score(), 2188189693529);
    }
}
