use miette::Result;
pub trait Solution<T> {
    fn part1(input: &str) -> Result<T>;
    fn part2(input: &str) -> Result<T>;
}
