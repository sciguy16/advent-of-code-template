use color_eyre::Result;
use std::{str::FromStr, time::Instant};

const PUZZLE_INPUT: &str = include_str!("../input.txt");

struct DataType;

impl FromStr for DataType {
    type Err = color_eyre::Report;

    fn from_str(_inp: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self)
    }
}

fn part_one(_inp: &DataType) -> u64 {
    0
}

fn part_two(_inp: &DataType) -> u64 {
    0
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let data = PUZZLE_INPUT.parse()?;

    let start = Instant::now();
    let ans = part_one(&data);
    let elapsed = start.elapsed();
    println!("part one: {} in {} ms", ans, elapsed.as_secs_f32() * 1000.0);

    let start = Instant::now();
    let ans = part_two(&data);
    let elapsed = start.elapsed();
    println!("part two: {} in {} ms", ans, elapsed.as_secs_f32() * 1000.0);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "";

    #[test]
    fn test_part_1() {
        let inp = TEST_DATA.parse().unwrap();
        let ans = part_one(&inp);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_1_b() {
        let inp = PUZZLE_INPUT.parse().unwrap();
        let ans = part_one(&inp);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_2() {
        let inp = TEST_DATA.parse().unwrap();
        let ans = part_two(&inp);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_2_b() {
        let inp = PUZZLE_INPUT.parse().unwrap();
        let ans = part_two(&inp);
        assert_eq!(ans, 0);
    }
}
