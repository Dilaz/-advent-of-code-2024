use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../inputs/day5.txt");
    println!("Part 1: {}", &part1(input));
    println!("Part 2: {}", &part2(input));

    divan::main();
}

fn page_compare(a: &u32, b: &u32, rules: &BTreeSet<(u32, u32)>) -> std::cmp::Ordering {
    if rules.contains(&(*a, *b)) {
        std::cmp::Ordering::Less
    } else if rules.contains(&(*b, *a)) {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn is_sorted(pages: &[u32], rules: &BTreeSet<(u32, u32)>) -> bool {
    pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
}

pub fn part1(input: &str) -> u32 {
    let mut rules = vec![];
    let mut split = input.split("\n\n");
    for line in split.next().unwrap().lines() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");
        rules.push((parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap().parse::<u32>().unwrap()));
    }
    let rules_set = rules.into_iter().collect::<BTreeSet<_>>();

    split.next().unwrap().lines()
    .map(|line| line.split(",").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>())
    .filter(|pages| is_sorted(pages, &rules_set))
    .map(|line| *line.get(line.len() / 2).unwrap())
    .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut rules = vec![];
    let mut split = input.split("\n\n");
    for line in split.next().unwrap().lines() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");
        rules.push((parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap().parse::<u32>().unwrap()));
    }
    let rules_set = rules.into_iter().collect::<BTreeSet<_>>();

    split.next().unwrap().lines()
    .map(|line| line.split(",").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>())
    .filter(|pages| !is_sorted(pages, &rules_set))
    .map(|pages| {
        let mut pages = pages.to_owned();
        let index = pages.len() / 2;
        let (_, median, _) = pages.select_nth_unstable_by(index, |a, b| page_compare(a, b, &rules_set));

        *median
    })
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
