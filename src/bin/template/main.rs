mod solution;
use solution::DayX;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/dayX.txt");
    println!("Part 1: {:?}", DayX::part1(input));
    println!("Part 2: {:?}", DayX::part2(input));
}
