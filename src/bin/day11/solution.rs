#[path = "../../utils.rs"]
pub mod utils;
use std::collections::HashMap;

pub use utils::Solution;
use miette::Result;
pub struct Day11;

const PART_1_BLINKS: u64 = 25;
const PART_2_BLINKS: u64 = 75;

trait NumLen {
    fn len(&self) -> u64;
}

impl NumLen for u64 {
    fn len(&self) -> u64 {
        self.ilog10() as u64 + 1
    }
}

pub fn solve(stone: u64, n: u64, max_iterations: u64, cache: &mut HashMap<(u64, u64, u64), u64>) -> u64 {
    if max_iterations == n {
        return 1;
    } else if let Some(cache_value ) = cache.get(&(stone, n, max_iterations)) {
        return *cache_value;
    }

    let result = 
    if stone == 0 {
        solve(1, n + 1, max_iterations, cache)
    } else if stone.len() % 2 == 0 {
        let mut new_stone = 0u64;
        let mut stone = stone;
        for i in 0..stone.len() / 2 {
            new_stone += stone % 10 * 10u64.pow(i as u32);
            stone /= 10;
        }
        solve(stone, n + 1, max_iterations, cache) + solve(new_stone, n + 1, max_iterations, cache)
    } else {
        solve(stone * 2024, n + 1, max_iterations, cache)
    };

    cache.insert((stone, n, max_iterations), result);
    result
}

impl Solution<u64> for Day11 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u64> {
        let stones = input
            .split_whitespace()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut cache = HashMap::new();

        Ok(stones.into_iter().map(|stone| solve(stone, 0, PART_1_BLINKS, &mut cache)).sum())
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u64> {
        let stones = input
            .split_whitespace()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut cache = HashMap::new();

        Ok(stones.into_iter().map(|stone| solve(stone, 0, PART_2_BLINKS, &mut cache)).sum())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day11, Solution};

    #[test]
    fn test_part1() {
        let test = r#"125 17"#;
        let result = Day11::part1(test);
        assert_eq!(result.unwrap(), 55312)
    }

    #[test]
    fn test_part2() {
        let test = r#"125 17"#;
        let result = Day11::part2(test);
        assert_eq!(result.unwrap(), 65601038650482)
    }
}
