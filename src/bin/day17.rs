use std::error;
use std::ops::RangeInclusive;

type Area = (RangeInclusive<isize>, RangeInclusive<isize>);

/// Result of a probe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeResult {
    Hit,
    Miss,
    Uncertain,
}

/// Moving probe
#[derive(Debug)]
struct Probe {
    position: (isize, isize),
    velocity: (isize, isize),
    max_y: isize,
}

impl Probe {
    /// Create (shoot) new probe
    fn new(velocity: (isize, isize)) -> Self {
        let position = (0, 0);
        Self {
            position,
            velocity,
            max_y: position.1,
        }
    }

    /// Next movement step
    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.velocity.0 -= self.velocity.0.signum();
        self.position.1 += self.velocity.1;
        self.velocity.1 -= 1;
        self.max_y = self.max_y.max(self.position.1);
    }

    /// Check probe reaching target area
    ///   Ok(true): inside target area
    ///   Ok(false): outside target area but moving towards
    ///   Err(()): outside target area and moving away from it
    fn check_target(&self, target_area: &Area) -> ProbeResult {
        if target_area.0.contains(&self.position.0) && target_area.1.contains(&self.position.1) {
            ProbeResult::Hit
        } else if (self.position.0 < *target_area.0.start() && self.velocity.0 > 0)
            || (self.position.0 > *target_area.0.end() && self.velocity.0 < 0)
            || (self.position.1 < *target_area.1.start() && self.velocity.1 > 0)
            || (self.position.1 > *target_area.1.end())
        {
            ProbeResult::Uncertain
        } else {
            ProbeResult::Miss
        }
    }
}

/// Fire a probe with the given velocity and report steps needed, last position
/// and max height if it hits
fn fire(velocity: (isize, isize), target_area: &Area) -> Option<(usize, (isize, isize), isize)> {
    let mut probe = Probe::new(velocity);
    for i in 0..400 {
        probe.step();
        match probe.check_target(target_area) {
            ProbeResult::Hit => return Some((i + 1, probe.position, probe.max_y)),
            ProbeResult::Miss => return None,
            ProbeResult::Uncertain => (),
        }
    }
    None
}

/// Brute-force number of distinct velocities with probe hits and max height
fn brute_force_hits(target_area: &Area) -> Option<((isize, isize), isize, usize)> {
    let mut top = None;
    let mut hits = 0;
    for vx in -200..200 {
        for vy in -200..200 {
            if let Some((_n, _pos, y)) = fire((vx, vy), target_area) {
                hits += 1;
                if top.is_none() || matches!(top, Some((_velocity, max_y)) if y > max_y) {
                    top = Some(((vx, vy), y));
                }
            }
        }
    }
    top.map(|(velocity, max_y)| (velocity, max_y, hits))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    const TARGET_AREA: Area = (57..=116, -198..=-148);

    let ((_vx, _vy), max_y, hits) = brute_force_hits(&TARGET_AREA).ok_or("No solution")?;
    println!("Max probe height: {}", max_y);
    println!("Number of initial velocities with hits: {}", hits);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_AREA: Area = (20..=30, -10..=-5);

    #[test]
    fn part_1a() {
        assert_eq!(fire((7, 2), &TARGET_AREA), Some((7, (28, -7), 3)));
    }

    #[test]
    fn part_1b() {
        assert_eq!(fire((6, 3), &TARGET_AREA), Some((9, (21, -9), 6)));
    }

    #[test]
    fn part_1c() {
        assert_eq!(fire((9, 0), &TARGET_AREA), Some((4, (30, -6), 0)));
    }

    #[test]
    fn part_1d() {
        assert_eq!(fire((17, -4), &TARGET_AREA), None);
    }

    #[test]
    fn part_1e() {
        assert_eq!(fire((6, 9), &TARGET_AREA), Some((20, (21, -10), 45)));
    }

    #[test]
    fn part_2() {
        assert_eq!(brute_force_hits(&TARGET_AREA), Some(((6, 9), 45, 112)));
    }
}
