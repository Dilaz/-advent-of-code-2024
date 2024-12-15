#[path = "../../utils.rs"]
pub mod utils;
use std::collections::HashSet;
use glam::IVec2;
pub use utils::Solution;
use miette::Result;
use itertools::Itertools;

pub struct Day12;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
];

type AoCMap = Vec<Vec<char>>;

fn get_perimeters(map: &AoCMap, chr: char, position: IVec2, visited: &mut Vec<IVec2>) -> u32 {
    if map[position.y as usize][position.x as usize] != chr {
        return 1;
    } else if visited.contains(&position) {
        return 0;
    }

    visited.push(position);

    DIRECTIONS.iter().map(|dir| {
        let pos = position + *dir;
        if pos.x >= 0 && pos.x < map[0].len() as i32 && pos.y >= 0 && pos.y < map.len() as i32 {
            get_perimeters(map, chr, pos, visited)
        } else {
            1
        }
    }).sum()
}

fn calc_corners(coord: &IVec2, group: &[IVec2], map_height: i32, map_width: i32) -> u32 {
    let mut corners = 0;
    for (v1, v2) in DIRECTIONS.iter().circular_tuple_windows() {
        let current = coord + v1;
        let next: IVec2 = coord + v2;
        let current_is_in_map = current.x >= 0 && current.y >= 0 && current.x < map_width && current.y < map_height;
        let next_is_in_map = next.x >= 0 && next.y >= 0 && next.x < map_width && next.y < map_height;
        let sum = coord + v1 + v2;
        let sum_is_in_map = sum.x >= 0 && sum.y >= 0 && sum.x < map_width && sum.y < map_height;
        if current_is_in_map && next_is_in_map && group.contains(&current) && group.contains(&next) && !group.contains(&sum) && sum_is_in_map
        || (!group.contains(&current) && !group.contains(&next)) {
            corners += 1;
        }
    }

    corners
}

impl Solution<u32> for Day12 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        let map = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>()).collect::<AoCMap>();

        let mut all_visited = HashSet::<IVec2>::new();
        let mut result = 0;

        map.iter().enumerate().for_each(|(y, row)| row.iter().enumerate().for_each(|(x, chr)| {
            let pos = IVec2::new(x as i32, y as i32);
            if !all_visited.contains(&pos) {
                let mut visited = Vec::<IVec2>::new();
                let perimeters = get_perimeters(&map, *chr, pos, &mut visited);
                let area = visited.len() as u32;
                result += area * perimeters;

                visited.iter().for_each(|v| { all_visited.insert(*v); });
            }
            
        }));

        Ok(result)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        let map = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>()).collect::<AoCMap>();

        let map_height = map.len() as i32;
        let map_width = map[0].len() as i32;

        let mut result = 0;
        let mut groups = vec![];
        let mut all_visited = HashSet::<IVec2>::new();

        map.iter().enumerate().for_each(|(y, row)| row.iter().enumerate().for_each(|(x, chr)| {
            let pos = IVec2::new(x as i32, y as i32);
            if !all_visited.contains(&pos) {
                let mut visited = Vec::<IVec2>::new();
                let perimeters = get_perimeters(&map, *chr, pos, &mut visited);
                let area = visited.len() as u32;
                result += area * perimeters;

                visited.iter().for_each(|v| { all_visited.insert(*v); });
                groups.push(visited);
            }
            
        }));

        Ok(groups.into_iter().map(|group| {
            let group_size = group.len() as u32;
            let corners = group.iter().map(|v| calc_corners(v, &group, map_height, map_width)).sum::<u32>();
            corners * group_size
        }).sum())
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day12, Solution};

    #[test]
    fn test_part1_small() {
        let test = r#"AAAA
BBCD
BBCC
EEEC"#;
        let result = Day12::part1(test);
        assert_eq!(result.unwrap(), 140)
    }

    #[test]
    fn test_part1_large() {
        let test = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let result = Day12::part1(test);
        assert_eq!(result.unwrap(), 1930)
    }

    #[test]
    fn test_part2_small() {
        let test = r#"AAAA
BBCD
BBCC
EEEC"#;
        let result = Day12::part2(test);
        assert_eq!(result.unwrap(), 80)
    }

    #[test]
    fn test_part2_large_1() {
        let test = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        let result = Day12::part2(test);
        assert_eq!(result.unwrap(), 236)
    }

    #[test]
    fn test_part2_large_2() {
        let test = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
        let result = Day12::part2(test);
        assert_eq!(result.unwrap(), 368)
    }
}
