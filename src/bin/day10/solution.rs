#[path = "../../utils.rs"]
pub mod utils;

use glam::IVec2;
pub use utils::Solution;
use miette::Result;
use itertools::Itertools;
pub struct Day10;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn to_delta(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::NEG_Y,
            Self::Down => IVec2::Y,
            Self::Left => IVec2::NEG_X,
            Self::Right => IVec2::X,
        }
    }
}

pub fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<IVec2>) {
    let mut starts = vec![];
    let map = input.lines().enumerate()
    .map(|(y, l)| l.chars().enumerate()
        .map(|(x, n)| {
            if n == '0' {
                starts.push(IVec2::new(x as i32, y as i32));
            }
            n.to_digit(10).unwrap()
        }).collect()).collect();

    (map, starts)
}

pub fn find_path(map: &Vec<Vec<u32>>, start_pos: &IVec2, current_num: u32, goals: &mut Vec<IVec2>) {
    DIRECTIONS.iter().for_each(|dir| {
        let next_pos = start_pos + dir.to_delta();
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= map[0].len() as i32 || next_pos.y >= map.len() as i32 {
            return;
        }
        let next_num = map[next_pos.y as usize][next_pos.x as usize];
        if current_num == 8 && next_num == 9 {
            goals.push(next_pos);
            return;
        } else if next_num != current_num + 1 {
            return;
        }
        find_path(map, &next_pos, next_num, goals);
    });
}

impl Solution<u32> for Day10 {
    fn part1(input: &str) -> Result<u32> {
        let (map, starts) = parse(input);

        Ok(starts.into_iter()
        .map(|start| {
            let mut goals: Vec<IVec2> = vec![];
            find_path(&map, &start, 0, &mut goals);
            goals.into_iter().unique().count() as u32
        })
        .sum::<u32>())
    }

    fn part2(input: &str) -> Result<u32> {
        let (map, starts) = parse(input);

        Ok(starts.into_iter()
        .map(|start| {
            let mut goals: Vec<IVec2> = vec![];
            find_path(&map, &start, 0, &mut goals);
            goals.len() as u32
        })
        .sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day10, Solution};

    #[test]
    fn test_part1_small() {
        let test = r#"0123
1234
8765
9876"#;
        let result = Day10::part1(test);
        assert_eq!(result.unwrap(), 1)
    }

    #[test]
    fn test_part1_big() {
        let test = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let result = Day10::part1(test);
        assert_eq!(result.unwrap(), 36)
    }

    #[test]
    fn test_part2_small() {
        let test = r#"012345
123456
234567
345678
416789
567891"#;
        let result = Day10::part2(test);
        assert_eq!(result.unwrap(), 227)
    }

    #[test]
    fn test_part2_big() {
        let test = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let result = Day10::part2(test);
        assert_eq!(result.unwrap(), 81)
    }
}
