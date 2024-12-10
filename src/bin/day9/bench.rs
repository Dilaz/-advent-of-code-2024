mod solution;
use solution::parse_part1;
use solution::parse_part2;
use solution::Day9;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u64> {
    Day9::part1(divan::black_box(include_str!(
        "../../../inputs/day9.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u64> {
    Day9::part2(divan::black_box(include_str!(
        "../../../inputs/day9.txt",
    )))
}

#[divan::bench]
fn bench_parse_part1() -> Result<()> {
    parse_part1(divan::black_box(include_str!(
        "../../../inputs/day9.txt",
    )));

    Ok(())
}

#[divan::bench]
fn bench_parse_part2() -> Result<()> {
    parse_part2(divan::black_box(include_str!(
        "../../../inputs/day9.txt",
    )));

    Ok(())
}
