use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../inputs/day3.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse_part1(input: &str) -> Vec<Instruction> {
    many1(many_till(anychar, mul))(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(_, i)| i)
        .collect()
}

fn part1(input: &str) -> u32 {
    parse_part1(&input)
        .into_iter()
        .map(|i| match i {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn parse_part2(input: &str) -> Vec<Instruction> {
    many1(many_till(
        anychar,
        alt((
            mul,
            value(Instruction::Do, tag("do()")),
            value(Instruction::Dont, tag("don't()")),
        )),
    ))(input)
    .unwrap()
    .1
    .into_iter()
    .map(|(_, i)| i)
    .collect()
}

fn part2(input: &str) -> u32 {
    parse_part2(&input)
        .into_iter()
        .inspect(|i| println!("{:?}", i))
        .fold((Instruction::Do, 0), |acc, i| match (acc.0, i) {
            (Instruction::Do, Instruction::Mul(a, b)) => (acc.0, acc.1 + a * b),
            (_, Instruction::Dont) => (Instruction::Dont, acc.1),
            (_, Instruction::Do) => (Instruction::Do, acc.1),
            _ => acc,
        })
        .1
}

#[test]
fn test_part1() {
    let test =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#.to_string();
    let result = part1(&test);
    assert_eq!(result, 161)
}

#[test]
fn test_part2() {
    let test =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#.to_string();
    let result = part2(&test);
    assert_eq!(result, 48)
}
