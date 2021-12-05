//! Advent of Code: puzzle input reading

#![allow(clippy::missing_errors_doc)]

use itertools::Itertools;
use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

/// Path to puzzle input files
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input");

/// Puzzle input
#[derive(Debug)]
pub struct Input {
    reader: BufReader<File>,
}

// Constructors
impl Input {
    /// Open puzzle input for the given day
    pub fn day(day: usize) -> io::Result<Self> {
        Self::open(&format!("day{:02}", day))
    }

    /// Open puzzle input with the given name
    pub fn open(name: &str) -> io::Result<Self> {
        let mut filename: PathBuf = INPUT_PATH.into();
        filename.push(name);
        filename.set_extension("txt");
        let reader = BufReader::new(File::open(filename)?);
        Ok(Input { reader })
    }
}

// Consuming all input
impl Input {
    /// Iterator over lines of this input
    pub fn lines(self) -> impl Iterator<Item = io::Result<String>> {
        self.reader.lines()
    }

    /// Iterator over parsed lines of this input
    pub fn parsed_lines<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.lines().map(|line| {
            line.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }

    /// Iterator over blocks of this input
    pub fn blocks(self) -> impl Iterator<Item = io::Result<Vec<String>>> {
        fn is_blank_line(line: &io::Result<String>) -> bool {
            line.as_ref().map(|s| s.trim().is_empty()).unwrap_or(false)
        }
        fn is_not_blank_line(line: &io::Result<String>) -> bool {
            !is_blank_line(line)
        }

        self.reader.lines().batching(|lines| {
            let block: io::Result<Vec<_>> = lines
                .skip_while(is_blank_line)
                .take_while(is_not_blank_line)
                .try_collect();
            match block {
                Ok(ref lines) if !lines.is_empty() => Some(block),
                _ => None,
            }
        })
    }
}

// Consuming partial input
impl Input {
    /// Read one line
    pub fn line(&mut self) -> io::Result<String> {
        self.reader
            .by_ref()
            .lines()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Input exhausted"))?
    }

    /// Read and parse one line
    pub fn parse_line<T>(&mut self) -> io::Result<T>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.line()?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file() {
        let mut lines = Input::day(1).unwrap().lines();
        let _line = lines.next().unwrap().unwrap();
    }

    #[test]
    fn lines() {
        let lines: Vec<_> = Input::open("test-numbers")
            .unwrap()
            .lines()
            .try_collect()
            .unwrap();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], "11");
        assert_eq!(lines[1], "22");
        assert_eq!(lines[2], "33");
        assert_eq!(lines[3], "44");
        assert_eq!(lines[4], "55");
    }

    #[test]
    fn parsed_lines() {
        let lines: Vec<u32> = Input::open("test-numbers")
            .unwrap()
            .parsed_lines()
            .try_collect()
            .unwrap();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], 11);
        assert_eq!(lines[1], 22);
        assert_eq!(lines[2], 33);
        assert_eq!(lines[3], 44);
        assert_eq!(lines[4], 55);
    }

    #[test]
    fn blocks() {
        let blocks: Vec<_> = Input::open("test-blocks")
            .unwrap()
            .blocks()
            .try_collect()
            .unwrap();
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].len(), 2);
        assert_eq!(blocks[0][0], "11");
        assert_eq!(blocks[0][1], "22");
        assert_eq!(blocks[1].len(), 2);
        assert_eq!(blocks[1][0], "33");
        assert_eq!(blocks[1][1], "44");
        assert_eq!(blocks[2].len(), 2);
        assert_eq!(blocks[2][0], "55");
        assert_eq!(blocks[2][1], "66");
    }

    #[test]
    fn partial_line() {
        let mut input = Input::open("test-numbers").unwrap();
        assert_eq!(input.line().unwrap(), "11");
        assert_eq!(input.parse_line::<u32>().unwrap(), 22);
        assert_eq!(input.parse_line::<u32>().unwrap(), 33);
        assert_eq!(input.line().unwrap(), "44");
        assert_eq!(input.line().unwrap(), "55");
        assert!(input.line().is_err());
    }

    #[test]
    fn partial_rest() {
        let mut input = Input::open("test-numbers").unwrap();
        assert_eq!(input.line().unwrap(), "11");
        assert_eq!(input.line().unwrap(), "22");
        let lines: Vec<_> = input.lines().try_collect().unwrap();
        assert_eq!(lines[0], "33");
        assert_eq!(lines[1], "44");
        assert_eq!(lines[2], "55");
    }
}
