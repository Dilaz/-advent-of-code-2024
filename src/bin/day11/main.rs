mod solution;
use solution::Day11;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day11.txt");
    println!("Part 1: {:?}", Day11::part1(input));
    println!("Part 2: {:?}", Day11::part2(input));
}
