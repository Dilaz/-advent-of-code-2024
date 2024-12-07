use nom::{bytes::complete::tag, character::complete::{self, line_ending}, multi::separated_list1, sequence::separated_pair, IResult};
use rayon::prelude::*;

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));

    divan::main();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

#[allow(dead_code)]
trait Concat {
    fn concat(&self, other: u32) -> u64;
}
impl Concat for u64 {
    fn concat(&self, other: u32) -> u64 {
        let mut num = other;
        let mut mul = 1_u32;
        while num > 0 {
            num /= 10;
            mul *= 10;
        }

        *self * mul as u64 + other as u64
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u32>)>> {
    let result = separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(
                tag(" "),
                 complete::u32
            )
    ))(input)?
    .1
    .into_iter()
    .map(|(a, b)| (a, b))
    .collect();

    Ok((input, result))
}

fn calculate(numbers: &[u32], sum: u64, result: u64, operations: &[Operation]) -> bool {
    let num = *numbers.first().unwrap();
    operations.iter().any(|operation| {
        let new_sum = match operation {
            Operation::Add => sum + num as u64,
            Operation::Multiply => sum * num as u64,
            Operation::Concat => (sum * 10_u64.pow(num.ilog10() + 1)) + num as u64,
            // Operation::Concat => sum.concat(num),
        };

        if new_sum == result && numbers.len() == 1 {
            return true;
        }
        else if new_sum > result || numbers.len() <= 1 {
            return false;
        }

        calculate(&numbers[1..], new_sum, result, operations)
    })
}

fn part1(input: &str) -> u64 {
    let operations = [Operation::Add, Operation::Multiply];
    parse(input)
    .unwrap()
    .1
    .into_par_iter()
    .filter(|(result, numbers)| {
        // If the result is smaller than the sum of all numbers (except for 1), it's impossible to reach the result
        let min = numbers.iter().map(|x| *x as u64).filter(|x| *x != 1).sum();
        if *result < min {
            return false
        } else if *result == min {
            return true
        }

        calculate(&numbers[1..], numbers[0] as u64, *result, &operations)
    })
    .map(|(result, _)| result)
    .sum()
}

fn part2(input: &str) -> u64 {
    let operations = [Operation::Add, Operation::Multiply, Operation::Concat];
    parse(input)
    .unwrap()
    .1
    .into_par_iter()
    .filter(|(result, numbers)| {
        let min = numbers.iter().map(|x| *x as u64).filter(|x| *x != 1).sum();
        if *result < min {
            return false
        } else if *result == min {
            return true
        }

        calculate(&numbers[1..], numbers[0] as u64, *result, &operations)
    })
    .map(|(result, _)| result)
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#.to_string();
        let result = part1(&test);
        assert_eq!(result, 3749)
    }

    #[test]
    fn test_part2() {
        let test = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#.to_string();
        let result = part2(&test);
        assert_eq!(result, 11387)
    }

    #[test]
    fn test_concat() {
        assert_eq!(12_u64.concat(345), 12345);
        assert_eq!(1_u64.concat(345), 1345);
        assert_eq!(123_u64.concat(45), 12345);
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day7.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day7.txt",
        )));
    }
}