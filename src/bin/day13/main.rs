mod solution;
use solution::Day13;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day13.txt");
    println!("Part 1: {:?}", Day13::part1(input));
    println!("Part 2: {:?}", Day13::part2(input));
}
