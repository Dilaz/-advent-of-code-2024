mod solution;
use solution::Day16;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day16.txt");
    println!("Part 1: {:?}", Day16::part1(input));
    println!("Part 2: {:?}", Day16::part2(input));
}
