mod solution;
use solution::Day10;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day10.txt");
    println!("Part 1: {:?}", Day10::part1(input));
    println!("Part 2: {:?}", Day10::part2(input));
}
