use advent_of_code_2021::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::error;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Bingo board
#[derive(Debug, Clone)]
struct Board {
    numbers: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl From<[[u8; 5]; 5]> for Board {
    fn from(numbers: [[u8; 5]; 5]) -> Self {
        Self {
            numbers,
            marks: [[false; 5]; 5],
        }
    }
}

impl<S: AsRef<str>> TryFrom<&[S]> for Board {
    type Error = ParseError;

    fn try_from(lines: &[S]) -> Result<Self, Self::Error> {
        fn parse_line(line: &str) -> Result<[u8; 5], ParseError> {
            let values: Vec<u8> = line
                .split_whitespace()
                .map(|s| s.parse())
                .try_collect()
                .map_err(|_| ParseError)?;
            values.try_into().map_err(|_| ParseError)
        }

        let rows: Vec<[u8; 5]> = lines
            .iter()
            .map(|line| parse_line(line.as_ref()))
            .try_collect()?;
        let numbers: [[u8; 5]; 5] = rows.try_into().map_err(|_| ParseError)?;

        Ok(Self::from(numbers))
    }
}

impl Board {
    /// Height of board
    const fn height(&self) -> usize {
        self.numbers.len()
    }

    /// Width of board
    const fn width(&self) -> usize {
        self.numbers[0].len()
    }

    /// Mark given number on board, return score if won
    fn mark(&mut self, number: u8) -> Option<u32> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.numbers[y][x] == number {
                    self.marks[y][x] = true;
                    if (0..self.height()).all(|y| self.marks[y][x])
                        || (0..self.width()).all(|x| self.marks[y][x])
                    {
                        return Some(self.score() * number as u32);
                    }
                }
            }
        }
        None
    }

    /// Calculate score (regardless of winning condition)
    fn score(&self) -> u32 {
        (0..self.height())
            .map(|y| {
                (0..self.width())
                    .filter(|x| !self.marks[y][*x])
                    .map(|x| self.numbers[y][x] as u32)
                    .sum::<u32>()
            })
            .sum()
    }
}

/// Bingo game
struct Game<'a> {
    boards: &'a mut [Board],
}

impl<'a> Game<'a> {
    /// Start new game
    fn new(boards: &'a mut [Board]) -> Self {
        Self { boards }
    }

    /// Play round with given number, return board and score of winners
    fn round(&mut self, number: u8) -> Vec<(usize, u32)> {
        self.boards
            .iter_mut()
            .enumerate()
            .filter_map(|(b, board)| board.mark(number).map(|score| (b, score)))
            .collect()
    }

    /// Play game with given sequence of numbers, return round, board and score of first winner
    fn play(&mut self, numbers: &[u8]) -> Option<(usize, usize, u32)> {
        for (r, number) in numbers.iter().enumerate() {
            if let Some((b, score)) = self.round(*number).first() {
                return Some((r, *b, *score));
            }
        }
        None
    }

    /// Play game with given sequence of numbers, return round, board and score of last winner
    fn play_last(&mut self, numbers: &[u8]) -> Option<(usize, usize, u32)> {
        let mut winners = HashSet::new();
        let mut last_winner = None;
        for (r, number) in numbers.iter().enumerate() {
            for (b, score) in self.round(*number) {
                if !winners.contains(&b) {
                    last_winner = Some((r, b, score));
                    winners.insert(b);
                }
            }
        }
        last_winner
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = Input::day(4)?;
    let numbers: Vec<u8> = input.line()?.split(',').map(|s| s.parse()).try_collect()?;

    let blocks: Vec<_> = input.blocks().try_collect()?;
    let mut boards: Vec<_> = blocks
        .into_iter()
        .map(|lines| Board::try_from(&lines[..]))
        .try_collect()?;

    let mut boards1 = boards.clone();
    let mut game = Game::new(&mut boards1);
    let (round, board, score) = game.play(&numbers).unwrap();
    println!(
        "Board {} wins in round {} with a score of {}",
        board, round, score
    );

    let mut game = Game::new(&mut boards);
    let (round, board, score) = game.play_last(&numbers).unwrap();
    println!(
        "Board {} wins LAST in round {} with a score of {}",
        board, round, score
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [u8; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    const BOARDS: [[[u8; 5]; 5]; 3] = [
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ],
        [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ],
    ];

    fn boards() -> [Board; 3] {
        BOARDS.map(Board::from)
    }

    #[test]
    fn part_1() {
        let mut boards = boards();
        let mut game = Game::new(&mut boards);
        assert_eq!(game.play(&NUMBERS), Some((11, 2, 4512)));
    }

    #[test]
    fn part_2() {
        let mut boards = boards();
        let mut game = Game::new(&mut boards);
        assert_eq!(game.play_last(&NUMBERS), Some((14, 1, 1924)));
    }
}
