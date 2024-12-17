#[path = "../../utils.rs"]
pub mod utils;
pub use utils::Solution;

use std::collections::HashSet;
use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag_collect};
use miette::Result;

pub struct Day16;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_delta(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::East  => IVec2::X,
            Direction::South => IVec2::Y,
            Direction::West  => IVec2::NEG_X,
        }
    }
}

type AoCMap = Vec<Vec<char>>;

const DIRECTIONS: [Direction; 4] = [ Direction::North, Direction::East, Direction::South, Direction::West ];
const STEP_SCORE: u32 = 1;
const TURN_SCORE: u32 = 1000;

fn find_successor(map: &AoCMap, coord: &IVec2, current_direction: &Direction) -> Vec<((Direction, IVec2), u32)> {
    DIRECTIONS.iter().filter_map(|dir| {
        let delta = dir.to_delta();
        let next = coord + delta;
        if map[next.y as usize][next.x  as usize] == '.' {
            return Some(if current_direction == dir {
                ((dir.clone(), next), STEP_SCORE)
            } else {
                ((dir.clone(), next), TURN_SCORE + STEP_SCORE)
            });
        }
        None
    }).collect_vec()
}

pub fn solve_part1(map: &AoCMap, start: &IVec2, end: &IVec2) -> u32 {
    let path = astar(
        &(Direction::East, *start),
        |(dir, coord)| {
            find_successor(map, coord, dir)
        },
        |(_, coord)| coord.distance_squared(*end) as u32,
        |(_, coord)| coord == end
    );

    dbg!(&path);

    if let Some((_, score)) = path {
        score
    } else {
        panic!();
    }
}

pub fn solve_part2(map: &AoCMap, start: &IVec2, end: &IVec2) -> u32 {
    let paths = astar_bag_collect(
        &(Direction::East, *start),
        |(dir, coord)| {
            find_successor(map, coord, dir)
        },
        |(_, coord)| coord.distance_squared(*end) as u32,
        |(_, coord)| coord == end
    );

    if let Some((paths, _)) = paths {
        println!("Found {} paths", paths.len());
        let points = paths.into_iter().flat_map(|path| {
            path.iter().map(|p| p.1).collect_vec()
        }).collect::<HashSet<IVec2>>();

        // print_map_paths(map, &points);

        points.len() as u32
    } else {
        panic!();
    }
}

#[allow(dead_code)]
fn print_map_paths(map: &[Vec<char>], points: &HashSet<IVec2>) {
    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, chr)| {
            if points.contains(&IVec2::new(x as i32, y as i32)) {
                print!("O");
            } else {
                print!("{}", chr);
            }
        });
        println!();
    });
    println!();
}

fn parse_map(input: &str) -> (AoCMap, IVec2, IVec2) {
    let (mut start, mut end) = (IVec2::ZERO, IVec2::ZERO);
    let map = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, chr)| {
            match chr {
                'S' => {
                    start = IVec2::new(x as i32, y as i32);
                    '.'
                }
                'E' => {
                    end = IVec2::new(x as i32, y as i32);
                    '.'
                }
                _ => chr,
            }
        }).collect_vec()
    })
    .collect::<AoCMap>();

    (map, start, end)
}

impl Solution<u32> for Day16 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        let (map, start, end) = parse_map(input);

        let score = solve_part1(&map, &start, &end);
        Ok(score)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        let (map, start, end) = parse_map(input);
    
        let score = solve_part2(&map, &start, &end);
        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use glam::IVec2;

    #[allow(unused_imports)]
    use super::{Day16, Solution, parse_map};

    #[test]
    fn test_parse_map() {
        let test = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let result = parse_map(test);
        assert_eq!(result.1, IVec2::new(1, 13));
        assert_eq!(result.2, IVec2::new(13, 1));
    }

    #[test]
    fn test_part1() {
        let test = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let result = Day16::part1(test);
        assert_eq!(result.unwrap(), 7036)
    }

    #[test]
    fn test_part1_2() {
        let test = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let result = Day16::part1(test);
        assert_eq!(result.unwrap(), 11048)
    }

    #[test]
    fn test_part2() {
        let test = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let result = Day16::part2(test);
        assert_eq!(result.unwrap(), 45)
    }

    #[test]
    fn test_part2_2() {
        let test = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let result = Day16::part2(test);
        assert_eq!(result.unwrap(), 64)
    }
}
