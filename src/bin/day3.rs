use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day3.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
    .map(|c| c.extract())
    .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
    .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut do_mul = true;
    re.captures_iter(input)
    .filter_map(|c| {
        if c.get(4).is_some() {
            do_mul = true;
            None
        } else if c.get(5).is_some() {
            do_mul = false;
            None
        } else if c.get(1).is_some() && do_mul {
            Some(c.get(2).unwrap().as_str().parse::<u32>().unwrap() * c.get(3).unwrap().as_str().parse::<u32>().unwrap())
        } else {
            None
        }
    })
    .sum()
}


#[test]
fn test_part1() {
    let test = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#.to_string();
    let result = part1(&test);
    assert_eq!(result, 161)
}

#[test]
fn test_part2() {
    let test = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#.to_string();
    let result = part2(&test);
    assert_eq!(result, 48)
}