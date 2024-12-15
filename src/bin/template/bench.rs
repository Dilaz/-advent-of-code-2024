mod solution;
use solution::DayX;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u32> {
   DayX::part1(divan::black_box(include_str!(
        "../../../inputs/dayX.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u32> {
    DayX::part2(divan::black_box(include_str!(
        "../../../inputs/dayX.txt",
    )))
}