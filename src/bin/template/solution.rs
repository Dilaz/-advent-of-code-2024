#[path = "../../utils.rs"]
pub mod utils;
pub use utils::Solution;
use miette::Result;
pub struct DayX;

impl Solution<u32> for DayX {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        Ok(0)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{DayX, Solution};

    #[test]
    fn test_part1() {
        let test = r#""#;
        let result = DayX::part1(test);
        assert_eq!(result.unwrap(), 0)
    }

    #[test]
    fn test_part2() {
        let test = r#""#;
        let result = DayX::part2(test);
        assert_eq!(result.unwrap(), 0)
    }
}
