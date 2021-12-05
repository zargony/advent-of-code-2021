use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;

fn count_increasing(iter: impl Iterator<Item = u32>) -> usize {
    iter.tuple_windows::<(_, _)>()
        .filter(|(a, b)| b > a)
        .count()
}

fn sliding_window_sum(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    iter.tuple_windows::<(_, _, _)>().map(|(a, b, c)| a + b + c)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let depths: Vec<u32> = Input::day(1)?.parsed_lines().try_collect()?;

    let increasing_depths = count_increasing(depths.iter().copied());
    println!("Increasing depths: {}", increasing_depths);

    let increasing_depths = count_increasing(sliding_window_sum(depths.iter().copied()));
    println!("Increasing sliding-window depths: {}", increasing_depths);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEPTHS: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1() {
        assert_eq!(count_increasing(DEPTHS.iter().copied()), 7);
    }

    #[test]
    fn part_2() {
        assert_eq!(
            count_increasing(sliding_window_sum(DEPTHS.iter().copied())),
            5
        );
    }
}
