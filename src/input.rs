//! Advent of Code: puzzle input reading

#![allow(clippy::missing_errors_doc)]

use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Fuse;
use std::path::PathBuf;
use std::str::FromStr;

/// Path to puzzle input files
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input");

/// Puzzle input
#[derive(Debug)]
pub struct Input {
    reader: BufReader<File>,
}

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

    /// Iterator of lines
    pub fn iter_lines(self) -> impl Iterator<Item = io::Result<String>> {
        self.reader.lines()
    }

    /// Vector of lines
    pub fn lines(self) -> io::Result<Vec<String>> {
        self.iter_lines().collect()
    }

    /// Iterator of parsed lines
    pub fn iter_parsed_lines<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_lines().map(|line| {
            line.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }

    /// Vector of parsed lines
    pub fn parsed_lines<T>(self) -> io::Result<Vec<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_parsed_lines().collect()
    }

    /// Iterator of newline separated blocks
    pub fn iter_blocks(self) -> impl Iterator<Item = io::Result<String>> {
        Blocks(self.iter_lines().fuse())
    }

    /// Vector of newline separated blocks
    pub fn blocks(self) -> io::Result<Vec<String>> {
        self.iter_blocks().collect()
    }

    /// Iterator of parsed newline separated blocks
    pub fn iter_parsed_blocks<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_blocks().map(|block| {
            block.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }

    /// Vector of parsed newline separated blocks
    pub fn parsed_blocks<T>(self) -> io::Result<Vec<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_parsed_blocks().collect()
    }
}

/// Iterator for newline separated blocks
#[derive(Debug)]
pub struct Blocks<I: Iterator<Item = io::Result<String>>>(Fuse<I>);

impl<I: Iterator<Item = io::Result<String>>> Iterator for Blocks<I> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        (&mut self.0)
            .take_while(|res| match res {
                Ok(line) if line.trim().is_empty() => false,
                Ok(_line) => true,
                Err(_e) => false,
            })
            .fold(None, |block, line| match (block, line) {
                (None, Ok(l)) => Some(Ok(l)),
                (Some(Ok(mut b)), Ok(l)) => {
                    b.push('\n');
                    b.push_str(&l);
                    Some(Ok(b))
                }
                (Some(Err(e)), _) | (_, Err(e)) => Some(Err(e)),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let lines = Input::open("test").unwrap().lines().unwrap();
        assert_eq!(lines.len(), 9);
        assert_eq!(lines[0], "one two");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "three four");
        assert_eq!(lines[3], "five six");
        assert_eq!(lines[4], "");
        assert_eq!(lines[5], "seven eight");
        assert_eq!(lines[6], "nine ten");
        assert_eq!(lines[7], "");
        assert_eq!(lines[8], "eleven twelve");
    }

    #[test]
    fn blocks() {
        let blocks = Input::open("test").unwrap().blocks().unwrap();
        assert_eq!(blocks.len(), 4);
        assert_eq!(blocks[0], "one two");
        assert_eq!(blocks[1], "three four\nfive six");
        assert_eq!(blocks[2], "seven eight\nnine ten");
        assert_eq!(blocks[3], "eleven twelve");
    }
}
