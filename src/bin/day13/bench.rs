mod solution;
use solution::parse;
use solution::Day13;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse() {
   let _ = parse(divan::black_box(include_str!(
        "../../../inputs/day13.txt",
    )));
}

#[divan::bench]
fn bench_part1() -> Result<u64> {
   Day13::part1(divan::black_box(include_str!(
        "../../../inputs/day13.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u64> {
    Day13::part2(divan::black_box(include_str!(
        "../../../inputs/day13.txt",
    )))
}