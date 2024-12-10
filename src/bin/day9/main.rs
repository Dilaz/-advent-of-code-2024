mod solution;
use solution::Day9;
use solution::Solution;
use miette::Result;

#[tracing::instrument]
fn main() -> Result<()>{
    tracing_subscriber::fmt::init();
    let input = include_str!("../../../inputs/day9.txt");
    println!("Part 1: {:?}", Day9::part1(input)?);
    println!("Part 2: {:?}", Day9::part2(input)?);

    Ok(())
}
