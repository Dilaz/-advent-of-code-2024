mod solution;
use solution::Day15;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day17.txt");
    println!("Part 1: {:?}", Day15::part1(input));
    println!("Part 2: {:?}", Day15::part2(input));
}
