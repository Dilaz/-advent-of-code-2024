mod solution;
use solution::Day12;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day12.txt");
    println!("Part 1: {:?}", Day12::part1(input));
    println!("Part 2: {:?}", Day12::part2(input));
}
