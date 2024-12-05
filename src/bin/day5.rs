use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../inputs/day5.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));

    divan::main();
}

fn is_invalid(first: u32, second: u32, rules: &BTreeSet<(u32, u32)>) -> bool {
    rules.contains(&(second, first))
}

pub fn part1(input: &str) -> u32 {
    let mut rules = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");
        rules.push((parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap().parse::<u32>().unwrap()));
    }
    let rules_set = rules.into_iter().collect::<BTreeSet<_>>();

    lines
    .map(|line| line.split(",").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>())
    .filter(|pages| {
        pages.iter().enumerate().all(|(i, page)| !pages.iter().skip(i + 1).any(|page2| is_invalid(*page, *page2, &rules_set)))
    })
    .map(|line| *line.get(line.len() / 2).unwrap())
    .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut rules = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");
        rules.push((parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap().parse::<u32>().unwrap()));
    }
    let rules_set = rules.into_iter().collect::<BTreeSet<_>>();

    lines
    .map(|line| line.split(",").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>())
    .filter(|pages| {
        !pages.iter().enumerate().all(|(i, page)| !pages.iter().skip(i + 1).any(|page2| is_invalid(*page, *page2, &rules_set)))
    })
    .map(|pages| {
        let mut pages = pages.clone();
        pages.sort_by(|a, b| {
            if rules_set.contains(&(*a, *b)) {
                std::cmp::Ordering::Less
            } else if rules_set.contains(&(*b, *a)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        pages
    })
    .map(|line| *line.get(line.len() / 2).unwrap())
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#.to_string();
        let result = part1(&test);
        assert_eq!(result, 143)
    }

    #[test]
    fn test_part2() {
        let test = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#.to_string();
        let result = part2(&test);
        assert_eq!(result, 123)
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day5.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day5.txt",
        )));
    }
}