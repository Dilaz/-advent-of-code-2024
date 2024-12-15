mod solution;
use solution::Day14;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day14.txt");
    println!("Part 1: {:?}", Day14::part1(input));
    println!("Part 2: {:?}", Day14::part2(input));
}
