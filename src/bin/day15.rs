use advent_of_code_2021::Input;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Map with risk levels of the ceiling
#[derive(Debug)]
struct Map(Vec<Vec<u8>>);

impl<S: AsRef<str>> TryFrom<&[S]> for Map {
    type Error = ParseError;

    fn try_from(lines: &[S]) -> Result<Self, Self::Error> {
        Ok(Self(
            lines
                .iter()
                .map(|line| {
                    line.as_ref()
                        .chars()
                        .map(|ch| {
                            ch.to_digit(10)
                                .ok_or(ParseError)
                                .and_then(|n| u8::try_from(n).map_err(|_| ParseError))
                        })
                        .try_collect()
                })
                .try_collect()?,
        ))
    }
}

impl Map {
    /// Find path with lowest risk sum (Dijkstra algorithm)
    fn pathfinder(&self) -> Option<usize> {
        #[derive(Debug, Clone, Default)]
        struct BestPath {
            risk: Option<usize>,
            from: Option<(usize, usize)>,
            done: bool,
        }

        let height = self.0.len();
        let width = self.0[height - 1].len();
        let mut bestpaths = vec![vec![BestPath::default(); width]; height];
        bestpaths[0][0].risk = Some(0);

        let next_undone_with_least_risk =
            |bestpaths: &[Vec<BestPath>]| -> Option<(usize, usize, usize)> {
                bestpaths
                    .iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .map(move |(x, bestpath)| (y, x, bestpath))
                    })
                    .filter_map(|(y, x, bestpath)| {
                        bestpath
                            .risk
                            .filter(|_risk| !bestpath.done)
                            .map(|risk| (y, x, risk))
                    })
                    .min_by_key(|(_y, _x, risk)| *risk)
            };

        while let Some((y, x, risk)) = next_undone_with_least_risk(&bestpaths) {
            bestpaths[y][x].done = true;
            if y == height - 1 && x == width - 1 {
                break;
            }
            for (neighbor_y, neighbor_x) in [
                (y < height - 1).then(|| (y + 1, x)),
                (x < width - 1).then(|| (y, x + 1)),
                (y > 0).then(|| (y - 1, x)),
                (x > 0).then(|| (y, x - 1)),
            ]
            .into_iter()
            .flatten()
            {
                let mut neighbor_bestpath = &mut bestpaths[neighbor_y][neighbor_x];
                if !neighbor_bestpath.done {
                    let new_neighbor_risk = risk + self.0[neighbor_y][neighbor_x] as usize;
                    if neighbor_bestpath.risk.is_none()
                        || new_neighbor_risk < neighbor_bestpath.risk.unwrap()
                    {
                        neighbor_bestpath.risk = Some(new_neighbor_risk);
                        neighbor_bestpath.from = Some((y, x));
                    }
                }
            }
        }

        bestpaths[height - 1][width - 1].risk
    }

    /// Enlarge map by a given factor in both direction
    fn enlarge(&mut self, factor: usize) {
        let new_map: Vec<Vec<u8>> = (0..factor)
            .flat_map(|yy| {
                self.0.iter().map(move |row| {
                    (0..factor)
                        .flat_map(|xx| {
                            row.iter()
                                .map(move |risk| (risk + yy as u8 + xx as u8 - 1) % 9 + 1)
                        })
                        .collect()
                })
            })
            .collect();
        self.0 = new_map;
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<_> = Input::day(15)?.lines().try_collect()?;

    let mut map = Map::try_from(&lines[..])?;
    println!("Lowest risk: {}", map.pathfinder().unwrap_or(0));

    map.enlarge(5);
    println!("Lowest risk (full map): {}", map.pathfinder().unwrap_or(0));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: [&str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    fn map() -> Map {
        Map::try_from(&MAP[..]).unwrap()
    }

    #[test]
    fn part_1() {
        let map = map();
        assert_eq!(map.pathfinder(), Some(40));
    }

    #[test]
    fn part_2() {
        let mut map = map();
        map.enlarge(5);
        assert_eq!(map.0.len(), 50);
        assert_eq!(map.0[0].len(), 50);
        assert_eq!(map.0[0][..10], [1, 1, 6, 3, 7, 5, 1, 7, 4, 2]);
        assert_eq!(map.0[0][10..20], [2, 2, 7, 4, 8, 6, 2, 8, 5, 3]);
        assert_eq!(map.0[0][20..30], [3, 3, 8, 5, 9, 7, 3, 9, 6, 4]);
        assert_eq!(map.0[0][30..40], [4, 4, 9, 6, 1, 8, 4, 1, 7, 5]);
        assert_eq!(map.0[0][40..], [5, 5, 1, 7, 2, 9, 5, 2, 8, 6]);
        assert_eq!(map.0[49][..10], [6, 7, 5, 5, 4, 8, 8, 9, 3, 5]);
        assert_eq!(map.0[49][40..], [1, 2, 9, 9, 8, 3, 3, 4, 7, 9]);
        assert_eq!(map.pathfinder(), Some(315));
    }
}
