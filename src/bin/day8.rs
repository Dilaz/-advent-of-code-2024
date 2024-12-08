use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day8.txt");
    println!("Part 1: {}", &part1(input));
    println!("Part 2: {}", &part2(input));

    divan::main();
}

fn count_antinodes(antennas: &BTreeMap<char, Vec<(isize, isize)>>, map_width: usize, map_height: usize, harmonics: bool) -> u32 {
    antennas.iter()
    .flat_map(|(_, coords)| {
        coords.iter().combinations(2).flat_map(|antennas| {
            let mut antennas = antennas.into_iter();
            let sat1 = antennas.next().unwrap();
            let sat2 = antennas.next().unwrap();

            let dx = sat1.0 - sat2.0;
            let dy = sat1.1 - sat2.1;
            
            if harmonics {
                let mut coords = vec![];
                let mut x = sat1.0;
                let mut y = sat1.1;

                while x < map_width as isize && x >= 0 && y < map_height as isize && y >= 0 {
                    coords.push((x, y));
                    x += dx;
                    y += dy;
                }

                let mut x = sat2.0;
                let mut y = sat2.1;

                while x < map_width as isize && x >= 0 && y < map_height as isize && y >= 0 {
                    coords.push((x, y));
                    x -= dx;
                    y -= dy;
                }


                coords
            } else {
                vec![(sat1.0 + dx, sat1.1 + dy), (sat2.0 - dx, sat2.1 - dy)]
            }
        })
        .filter(|coord| coord.0 >= 0 && coord.0 < map_width as isize && coord.1 >= 0 && coord.1 < map_height as isize)
    })
    .collect::<BTreeSet<(isize, isize)>>()
    .into_iter()
    .len() as u32
}

fn part1(input: &str) -> u32 {
    let mut antennas  = BTreeMap::<char, Vec<(isize, isize)>>::new();
    input.lines().enumerate()
    .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9'  => {
            antennas.entry(c).and_modify(|v| v.push((x as isize, y as isize))).or_insert(vec![(x as isize, y as isize)]);
        }
        _ => {}
    }));

    count_antinodes(&antennas, input.lines().count(), input.lines().next().unwrap().chars().count(), false)
}

fn part2(input: &str) -> u32 {
    let mut antennas  = BTreeMap::<char, Vec<(isize, isize)>>::new();
    input.lines().enumerate()
    .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9'  => {
            antennas.entry(c).and_modify(|v| v.push((x as isize, y as isize))).or_insert(vec![(x as isize, y as isize)]);
        }
        _ => {}
    }));

    count_antinodes(&antennas, input.lines().count(), input.lines().next().unwrap().chars().count(), true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#.to_string();
        let result = part1(&test);
        assert_eq!(result, 14)
    }

    #[test]
    fn test_part2() {
        let test = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#.to_string();
        let result = part2(&test);
        assert_eq!(result, 34)
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day8.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day8.txt",
        )));
    }
}