use advent_of_code_2021::Input;
use itertools::Itertools;
use std::collections::HashSet;
use std::error;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error)]
#[error("Input parse error")]
struct ParseError;

/// Floor height map
#[derive(Debug)]
struct HeightMap(Vec<Vec<u8>>);

impl<S: AsRef<str>> TryFrom<&[S]> for HeightMap {
    type Error = ParseError;

    fn try_from(heightmap: &[S]) -> Result<Self, Self::Error> {
        Ok(Self(
            heightmap
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

impl HeightMap {
    /// Get height at a given position if exists
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.0.get(y).and_then(|row| row.get(x).copied())
    }

    /// Check whether the given position is a low point (i.e. there's no adjacent lower point)
    fn is_low_point(&self, x: usize, y: usize) -> Option<bool> {
        let height = self.get(x, y)?;
        let left = (x > 0).then(|| self.get(x - 1, y)).flatten();
        let right = self.get(x + 1, y);
        let above = (y > 0).then(|| self.get(x, y - 1)).flatten();
        let below = self.get(x, y + 1);
        Some(
            [left, right, above, below]
                .iter()
                .map(|adjacent| adjacent.map(|h| h <= height).unwrap_or(false))
                .all(|is_lower| !is_lower),
        )
    }

    /// Get all low points
    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.is_low_point(x, y).unwrap_or(false) {
                    points.push((x, y));
                }
            }
        }
        points
    }

    /// Get risk sum of all low points
    fn low_points_total_risk(&self) -> u32 {
        self.low_points()
            .iter()
            .map(|(x, y)| match self.get(*x, *y) {
                Some(height) => height as u32 + 1,
                None => 0,
            })
            .sum()
    }

    /// Get all points of basin at the given point
    fn basin_points(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        fn recurse(
            heightmap: &HeightMap,
            points: &mut HashSet<(usize, usize)>,
            x: usize,
            y: usize,
        ) {
            if !points.contains(&(x, y)) {
                if let Some(height) = heightmap.get(x, y) {
                    if height < 9 {
                        points.insert((x, y));
                        if x > 0 {
                            recurse(heightmap, points, x - 1, y);
                        }
                        if y > 0 {
                            recurse(heightmap, points, x, y - 1);
                        }
                        recurse(heightmap, points, x + 1, y);
                        recurse(heightmap, points, x, y + 1);
                    }
                }
            }
        }

        let mut points = HashSet::new();
        recurse(self, &mut points, x, y);
        points
    }

    /// Multiply size of top 3 basin sizes
    fn top_basins_size_factor(&self) -> usize {
        let mut basin_sizes: Vec<_> = self
            .low_points()
            .iter()
            .map(|(x, y)| self.basin_points(*x, *y).len())
            .collect();
        basin_sizes.sort_by(|a, b| b.cmp(a));
        basin_sizes.iter().take(3).product()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<_> = Input::day(9)?.lines().try_collect()?;
    let heightmap = HeightMap::try_from(&lines[..])?;

    println!(
        "Low points total risk: {}",
        heightmap.low_points_total_risk(),
    );

    println!(
        "Top basins size factor: {}",
        heightmap.top_basins_size_factor(),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHTMAP: [&str; 5] = [
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    fn heightmap() -> HeightMap {
        HeightMap::try_from(&HEIGHTMAP[..]).unwrap()
    }

    #[test]
    fn part_1() {
        let heightmap = heightmap();
        assert_eq!(heightmap.low_points(), [(1, 0), (9, 0), (2, 2), (6, 4)]);
        assert_eq!(heightmap.low_points_total_risk(), 15);
    }

    #[test]
    fn part_2() {
        let heightmap = heightmap();
        assert_eq!(heightmap.basin_points(1, 0).len(), 3);
        assert_eq!(heightmap.basin_points(9, 0).len(), 9);
        assert_eq!(heightmap.basin_points(2, 2).len(), 14);
        assert_eq!(heightmap.basin_points(6, 4).len(), 9);
        assert_eq!(heightmap.top_basins_size_factor(), 1134);
    }
}
