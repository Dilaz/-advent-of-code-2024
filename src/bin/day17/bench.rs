mod solution;
use solution::Day15;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<String> {
   Day15::part1(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<String> {
    Day15::part2(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )))
}