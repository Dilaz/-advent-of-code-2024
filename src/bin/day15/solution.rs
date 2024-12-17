#[path = "../../utils.rs"]
pub mod utils;
pub use utils::Solution;
use core::panic;
use std::collections::HashMap;
use glam::IVec2;
use miette::Result;
use itertools::Itertools;

pub struct Day15;

#[derive(Debug)]
pub enum Obstacle {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Debug)]
pub enum Instruction {
    North,
    East,
    South,
    West,
}

impl Instruction {
    fn to_delta(&self) -> IVec2 {
        match self {
            Instruction::North => IVec2::NEG_Y,
            Instruction::East => IVec2::X,
            Instruction::South => IVec2::Y,
            Instruction::West => IVec2::NEG_X,
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Instruction::North | Instruction::South)
    }
}

pub type AoCMap = HashMap<IVec2, Obstacle>;

pub fn parse_map(input: &str) -> (IVec2, AoCMap) {
    let mut start = IVec2::ZERO;
    let mut map = AoCMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, chr)| {
            let current = IVec2::new(x as i32, y as i32);
            match chr {
                '#' => { map.insert(current, Obstacle::Wall); },
                'O' => { map.insert(current, Obstacle::Box); },
                '@' => { start = current; },
                _ => { }
            }
        });
    });

    (start, map)
}

pub fn parse_part2_map(input: &str) -> (IVec2, AoCMap) {
    let mut start = IVec2::ZERO;
    let mut map = AoCMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, chr)| {
            let x = x * 2;
            let current = IVec2::new(x as i32, y as i32);
            let next = IVec2::new(x as i32 + 1, y as i32);
            match chr {
                '#' => {
                    map.insert(current, Obstacle::Wall);
                    map.insert(next, Obstacle::Wall);
                },
                'O' => {
                    map.insert(current, Obstacle::BoxLeft);
                    map.insert(next, Obstacle::BoxRight);
                },
                '@' => { start = current; },
                _ => { }
            }
        });
    });

    (start, map)
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.chars().filter_map(|chr| match chr {
        '^' => Some(Instruction::North),
        '>' => Some(Instruction::East),
        'v' => Some(Instruction::South),
        '<' => Some(Instruction::West),
        _ => None,
    }).collect::<Vec<_>>()
}

fn can_move(map: &AoCMap, start: &IVec2, instruction: &Instruction) -> bool {
    let delta = instruction.to_delta();
    let next = start + delta;
    if let Some(obstacle) = map.get(&next) {
        match obstacle {
            Obstacle::Box => can_move(map, &next, instruction),
            Obstacle::BoxLeft if instruction.is_vertical() => {
                can_move(map, &next, instruction)
                && can_move(map, &(next + Instruction::East.to_delta()), instruction)
            },
            Obstacle::BoxRight if instruction.is_vertical() => {
                can_move(map, &next, instruction)
                && can_move(map, &(next + Instruction::West.to_delta()), instruction)
            },
            Obstacle::BoxLeft | Obstacle::BoxRight => can_move(map, &next, instruction),
            Obstacle::Wall => false,
        }
    } else {
        true
    }
}

fn make_move(map: &mut AoCMap, robot: &mut IVec2, instruction: &Instruction) -> bool {
    if !can_move(map, robot, instruction) {
        return false;
    }
    let delta = instruction.to_delta();

    // Find the next free slot
    let mut current = *robot;
    current += delta;
    let mut first_box: Option<IVec2> = None;
    while map.get(&current).is_some() {
        if first_box.is_none() {
            first_box = Some(current);
        }
        current += delta;
    }

    *robot += delta;

    // Move the first box to the first free space
    if let Some(first_box) = first_box {
        map.remove(&first_box);
        map.insert(current, Obstacle::Box);
    }

    true
}

fn make_part2_move(map: &mut AoCMap, robot: &mut IVec2, instruction: &Instruction) -> bool {
    if !can_move(map, robot, instruction) {
        return false;
    }
    let delta = instruction.to_delta();

    // Find the next free slot
    let mut current = *robot;
    current += delta;
    let mut boxes: Vec<Vec<_>> = vec![];
    'outer: loop {
        let prev_boxes = if boxes.is_empty() { vec![*robot] } else { boxes.iter().next_back().unwrap().clone() };
        let mut all_free = true;
        for b in prev_boxes {
            match map.get(&(b + delta)) {
                Some(Obstacle::BoxLeft) if instruction.is_vertical() => {
                    all_free = false;
                    boxes.push(vec![b + delta, b + delta + Instruction::East.to_delta()]);
                },
                Some(Obstacle::BoxRight) if instruction.is_vertical() => {
                    all_free = false;
                    boxes.push(vec![b + delta, b + delta + Instruction::West.to_delta()]);
                },
                Some(Obstacle::BoxLeft) | Some(Obstacle::BoxRight) if !instruction.is_vertical() => {
                    all_free = false;
                    boxes.push(vec![b + delta]);
                },
                None => { }
                _ => { panic!(); }
            }
        }
        if all_free {
            break 'outer;
        }
    }

    *robot += delta;

    // Move the first box to the first free space
    boxes.into_iter().rev().unique().for_each(|b| {
        b.iter().for_each(|c| {
            if let Some(old) = map.remove(c) {
                let new_coord = c + delta;
                map.insert(new_coord, old);
            }
        });
    });

    // print_map(map, robot, IVec2::new(20, 10));

    true
}

fn print_map(map: &HashMap<IVec2, Obstacle>, robot: &IVec2, map_size: IVec2) {
    (0..map_size.y).for_each(|y| {
        (0..map_size.x).for_each(|x| {
            match map.get(&IVec2::new(x, y)) {
                Some(Obstacle::Box) => print!("O"),
                Some(Obstacle::BoxLeft) => print!("["),
                Some(Obstacle::BoxRight) => print!("]"),
                Some(Obstacle::Wall) => print!("#"),
                _ if robot.x == x && robot.y == y => print!("@"),
                _ => print!("."),
            }
        });
        println!();
    });
    println!();

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
}

fn calc_score(map: &AoCMap) -> u32 {
    map.iter().map(|(c, obj)| {
        match obj {
            Obstacle::Box | Obstacle::BoxLeft => (c.y * 100 + c.x) as u32,
            _ => 0
        }
    }).sum()
}

impl Solution<u32> for Day15 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        let mut parts = input.split("\n\n");
        let (mut robot, mut map) = parse_map(parts.next().unwrap());
        let instructions = parse_instructions(parts.next().unwrap());

        instructions.into_iter().for_each(|i| {
            make_move(&mut map, &mut robot, &i);
        });

        let score = calc_score(&map);

        Ok(score)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        let mut parts = input.split("\n\n");
        let (mut robot, mut map) = parse_part2_map(parts.next().unwrap());
        let instructions = parse_instructions(parts.next().unwrap());

        instructions.into_iter().for_each(|i| {
            make_part2_move(&mut map, &mut robot, &i);
        });

        let score = calc_score(&map);

        Ok(score)
        // 1274891 too low
        // 1399853 too low
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day15, Solution, parse_map};

    #[test]
    fn test_part1_small() {
        let test = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let result = Day15::part1(test);
        assert_eq!(result.unwrap(), 2028)
    }

    #[test]
    fn test_part1_large() {
        let test = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let result = Day15::part1(test);
        assert_eq!(result.unwrap(), 10092)
    }

    #[test]
    fn test_part2_small() {
        let test = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        let result = Day15::part2(test);
        assert_eq!(result.unwrap(), 618)
    }

    #[test]
    fn test_part2_large() {
        let test = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let result = Day15::part2(test);
        assert_eq!(result.unwrap(), 9021)
    }

}
