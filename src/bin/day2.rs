enum Direction {
    Ascending,
    Descending,
}

fn main() {
    let input = include_str!("../../inputs/day2.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .map(|word| word.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            is_safe(&numbers)
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .map(|word| word.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (0..numbers.len())
            .into_iter()
            .any(|i| {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                is_safe(&numbers)
            })
        })
        .count() as u32
}

fn is_safe(numbers: &Vec<u32>) -> bool {
    let mut direction: Option<Direction> = None;
    let mut prev: Option<u32> = None;

    for number in numbers {
        if prev.is_none() {
            prev = Some(*number);
            continue;
        }

        let diff = number.abs_diff(prev.unwrap());

        if diff > 3 || diff == 0 {
            return false;
        }

        match direction {
            None => {
                if number > &prev.unwrap() {
                    direction = Some(Direction::Ascending);
                } else {
                    direction = Some(Direction::Descending);
                }
            }
            Some(Direction::Ascending) => {
                if number < &prev.unwrap() {
                    return false;
                }
            }
            Some(Direction::Descending) => {
                if number > &prev.unwrap() {
                    return false;
                }
            }
        }
        prev = Some(*number);
    }

    true
}

#[test]
fn test_part1() {
    let test = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
        .to_string();
    let result = part1(&test);
    assert_eq!(result, 2)
}

#[test]
fn test_part2() {
    let test = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#
        .to_string();
    let result = part2(&test);
    assert_eq!(result, 4)
}