mod solution;
use solution::Day12;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u32> {
   Day12::part1(divan::black_box(include_str!(
        "../../../inputs/day12.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u32> {
    Day12::part2(divan::black_box(include_str!(
        "../../../inputs/day12.txt",
    )))
}