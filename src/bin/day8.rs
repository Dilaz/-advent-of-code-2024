use std::collections::{BTreeMap, HashSet};
use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day8.txt");
    println!("Part 1: {}", &part1(input));
    println!("Part 2: {}", &part2(input));

    divan::main();
}

fn count_antinodes(antennas: &BTreeMap<char, Vec<IVec2>>, map_width: i32, map_height: i32, harmonics: bool) -> u32 {
    antennas.iter()
    .flat_map(|(_, coords)| {
        coords.iter().tuple_combinations().flat_map(|(antenna1, antenna2)| {
            let delta = antenna1 - antenna2;
            
            if harmonics {
                let mut coords: Vec<IVec2> = vec![];
                let mut c = *antenna1;

                while c.x < map_width && c.x >= 0 && c.y < map_height && c.y >= 0 {
                    coords.push(c);
                    c += delta;
                }

                let mut c = *antenna2;

                while c.x < map_width && c.x >= 0 && c.y < map_height && c.y >= 0 {
                    coords.push(c);
                    c -= delta;
                }

                coords
            } else {
                vec![antenna1 + delta, antenna2 - delta]
            }
        })
        .filter(|c| c.x >= 0 && c.x < map_width && c.y >= 0 && c.y < map_height)
    })
    .collect::<HashSet<IVec2>>()
    .into_iter()
    .len() as u32
}

fn part1(input: &str) -> u32 {
    let antennas = parse_input(input);

    count_antinodes(&antennas, input.lines().count() as i32, input.lines().next().unwrap().chars().count() as i32, false)
}

fn part2(input: &str) -> u32 {
    let antennas = parse_input(input);

    count_antinodes(&antennas, input.lines().count() as i32, input.lines().next().unwrap().chars().count() as i32, true)
}

fn parse_input(input: &str) -> BTreeMap<char, Vec<IVec2>> {
    let mut antennas  = BTreeMap::<char, Vec<IVec2>>::new();
    input.lines().enumerate()
    .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9'  => {
            antennas.entry(c).and_modify(|v| v.push(IVec2::new(x as i32, y as i32))).or_insert(vec![IVec2::new(x as i32, y as i32)]);
        }
        _ => {}
    }));
    antennas
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
