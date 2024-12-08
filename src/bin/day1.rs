use std::{collections::HashMap, iter::zip};


fn main() {
    let input = include_str!("../../inputs/day1.txt");
    println!("Part 1: {}", &part1(input));
    println!("Part 2: {}", &part2(input));

    divan::main();
}

fn part1(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut words = line.split_whitespace();
        left.push(words.next().unwrap().parse::<i32>().unwrap());
        right.push(words.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    zip(left.iter(), right.iter())
    .map(|(l, r)| l.abs_diff(*r))
    .sum()
}

fn part2(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        left.push(words.next().unwrap().parse::<u32>().unwrap());
        right.entry(words.next().unwrap().parse::<u32>().unwrap()).and_modify(|f| *f += 1).or_insert(1);
    }

    left.into_iter()
    .fold(0, |acc, x| acc + x * *right.entry(x).or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let test = r#"3   4
4   3
2   5
1   3
3   9
3   3"#.to_string();
        let result = part1(&test);
        assert_eq!(result, 11)
    }

    #[test]
    fn test_part2() {
        let test = r#"3   4
4   3
2   5
1   3
3   9
3   3"#.to_string();
        let result = part2(&test);
        assert_eq!(result, 31)
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day1.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day1.txt",
        )));
    }
}
