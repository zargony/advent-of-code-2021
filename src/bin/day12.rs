use advent_of_code_2021::Input;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use std::{error, fmt};
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// A cave's name
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum CaveName {
    Start,
    Big(String),
    Small(String),
    End,
}

impl FromStr for CaveName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            s if s.chars().all(char::is_uppercase) => Self::Big(s.into()),
            s if s.chars().all(char::is_lowercase) => Self::Small(s.into()),
            _ => return Err(ParseError),
        })
    }
}

impl fmt::Display for CaveName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Small(ref name) => write!(f, "{}", name),
            Self::Big(ref name) => write!(f, "{}", name),
        }
    }
}

/// A system of interconnected caves
#[derive(Debug)]
struct Caves {
    paths: HashMap<CaveName, Vec<CaveName>>,
}

impl<S: AsRef<str>> TryFrom<&[S]> for Caves {
    type Error = ParseError;

    fn try_from(lines: &[S]) -> Result<Self, Self::Error> {
        let mut paths: HashMap<CaveName, Vec<CaveName>> = HashMap::new();
        for line in lines {
            let (name1, name2) = line.as_ref().split_once('-').ok_or(ParseError)?;
            let name1: CaveName = name1.parse()?;
            let name2: CaveName = name2.parse()?;
            paths.entry(name1.clone()).or_default().push(name2.clone());
            paths.entry(name2).or_default().push(name1);
        }
        for exits in paths.values_mut() {
            exits.sort();
        }
        Ok(Self { paths })
    }
}

impl Caves {
    /// Iterator over possible paths
    fn paths(&self) -> PathFinder<'_> {
        PathFinder::new(self)
    }

    /// Get possible exits of given cave
    fn possible_exits_for(&self, name: &CaveName) -> impl Iterator<Item = &CaveName> {
        self.paths
            .get(name)
            .map(|exits| exits.iter())
            .unwrap_or_else(|| [].iter())
    }
}

/// Cave path finder (iterator over possible paths)
struct PathFinder<'a> {
    /// Set of interconnected caves
    caves: &'a Caves,
    /// Current path
    path: Vec<CaveName>,
    /// Iterators of possible exits for every cave in current path
    exits: Vec<Box<dyn Iterator<Item = &'a CaveName> + 'a>>,
    /// Enable extra rule of part 2 (allow 1 small cave)
    extra: bool,
    /// Name of duplicate small cave in path
    dupe: Option<CaveName>,
}

impl<'a> PathFinder<'a> {
    /// Create new path finder for given caves
    fn new(caves: &'a Caves) -> Self {
        let mut pathfinder = Self {
            caves,
            path: Vec::new(),
            exits: Vec::new(),
            extra: false,
            dupe: None,
        };
        pathfinder.push(CaveName::Start);
        pathfinder
    }

    /// Enable extra rule of part 2 (allow 1 small cave)
    fn extra(mut self) -> Self {
        self.extra = true;
        self
    }

    /// Add next cave to path
    fn push(&mut self, name: CaveName) {
        let exits = self.caves.possible_exits_for(&name);
        self.path.push(name);
        self.exits.push(Box::new(exits));
    }

    /// Remove last cave from path
    fn pop(&mut self) {
        if self.dupe.as_ref() == self.path.last() {
            self.dupe = None;
        }
        self.path.pop();
        self.exits.pop();
    }

    /// Iterates to next cave to try
    fn next_cave(&mut self) -> Option<CaveName> {
        if let Some(CaveName::End) = self.path.last() {
            self.pop();
        }
        while !self.path.is_empty() {
            if let Some(last_cave_exits) = self.exits.last_mut() {
                for last_cave_next_exit in last_cave_exits {
                    let mut dupe = self.path.contains(last_cave_next_exit);
                    if dupe
                        && self.extra
                        && matches!(last_cave_next_exit, CaveName::Small(_))
                        && self.dupe.is_none()
                    {
                        self.dupe = Some(last_cave_next_exit.clone());
                        dupe = false;
                    }
                    if !dupe || matches!(last_cave_next_exit, CaveName::Big(_)) {
                        self.push(last_cave_next_exit.clone());
                        return Some(last_cave_next_exit.clone());
                    }
                }
                self.pop();
            }
        }
        None
    }
}

impl<'a> Iterator for PathFinder<'a> {
    type Item = Vec<CaveName>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.next_cave()?;
            if next == CaveName::End {
                return Some(self.path.clone());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<_> = Input::day(12)?.lines().try_collect()?;
    let caves = Caves::try_from(&lines[..])?;

    println!("Number of possible paths: {}", caves.paths().count());

    println!(
        "Number of possible paths with extra rule: {}",
        caves.paths().extra().count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAVES1: [&str; 7] = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    const CAVES2: [&str; 10] = [
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];
    const CAVES3: [&str; 18] = [
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    fn caves1() -> Caves {
        Caves::try_from(&CAVES1[..]).unwrap()
    }

    fn caves2() -> Caves {
        Caves::try_from(&CAVES2[..]).unwrap()
    }

    fn caves3() -> Caves {
        Caves::try_from(&CAVES3[..]).unwrap()
    }

    fn display_path(path: &[CaveName]) -> String {
        path.iter().map(|name| name.to_string()).join(",")
    }

    fn assert_next_path(paths: &mut PathFinder<'_>, s: &str) {
        assert_eq!(display_path(&paths.next().unwrap()), s);
    }

    #[test]
    fn part_1a() {
        let caves = caves1();
        let mut paths = caves.paths();
        assert_next_path(&mut paths, "start,A,b,A,c,A,end");
        assert_next_path(&mut paths, "start,A,b,A,end");
        assert_next_path(&mut paths, "start,A,b,end");
        assert_next_path(&mut paths, "start,A,c,A,b,A,end");
        assert_next_path(&mut paths, "start,A,c,A,b,end");
        assert_next_path(&mut paths, "start,A,c,A,end");
        assert_next_path(&mut paths, "start,A,end");
        assert_next_path(&mut paths, "start,b,A,c,A,end");
        assert_next_path(&mut paths, "start,b,A,end");
        assert_next_path(&mut paths, "start,b,end");
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn part_1b() {
        let caves = caves2();
        let mut paths = caves.paths();
        assert_next_path(&mut paths, "start,HN,dc,HN,kj,HN,end");
        assert_next_path(&mut paths, "start,HN,dc,HN,end");
        assert_next_path(&mut paths, "start,HN,dc,kj,HN,end");
        assert_next_path(&mut paths, "start,HN,dc,end");
        assert_next_path(&mut paths, "start,HN,kj,HN,dc,HN,end");
        assert_next_path(&mut paths, "start,HN,kj,HN,dc,end");
        assert_next_path(&mut paths, "start,HN,kj,HN,end");
        assert_next_path(&mut paths, "start,HN,kj,dc,HN,end");
        assert_next_path(&mut paths, "start,HN,kj,dc,end");
        assert_next_path(&mut paths, "start,HN,end");
        assert_next_path(&mut paths, "start,dc,HN,kj,HN,end");
        assert_next_path(&mut paths, "start,dc,HN,end");
        assert_next_path(&mut paths, "start,dc,kj,HN,end");
        assert_next_path(&mut paths, "start,dc,end");
        assert_next_path(&mut paths, "start,kj,HN,dc,HN,end");
        assert_next_path(&mut paths, "start,kj,HN,dc,end");
        assert_next_path(&mut paths, "start,kj,HN,end");
        assert_next_path(&mut paths, "start,kj,dc,HN,end");
        assert_next_path(&mut paths, "start,kj,dc,end");
        assert_eq!(paths.next(), None);
    }

    #[test]
    fn part_1c() {
        let caves = caves3();
        assert_eq!(caves.paths().count(), 226);
    }

    #[test]
    fn part_2a() {
        let caves = caves2();
        assert_eq!(caves.paths().extra().count(), 103);
    }

    #[test]
    fn part_2b() {
        let caves = caves3();
        assert_eq!(caves.paths().extra().count(), 3509);
    }
}
