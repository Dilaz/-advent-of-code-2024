mod solution;
use glam::IVec2;
use solution::parse_instructions;
use solution::parse_map;
use solution::AoCMap;
use solution::Day15;
use solution::Instruction;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1_parse_map() -> (IVec2, AoCMap) {
   parse_map(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )).split("\n\n").next().unwrap())
}

#[divan::bench]
fn bench_part1_parse_instructions() -> Vec<Instruction> {
   parse_instructions(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )).split("\n\n").nth(1).unwrap())
}

#[divan::bench]
fn bench_part1() -> Result<u32> {
   Day15::part1(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u32> {
    Day15::part2(divan::black_box(include_str!(
        "../../../inputs/day15.txt",
    )))
}