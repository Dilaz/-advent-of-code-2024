use std::collections::BTreeSet;

use rayon::prelude::*;

fn main() {
    let input = include_str!("../../inputs/day6.txt");
    println!("Part 1: {}", &part1(&input));
    println!("Part 2: {}", &part2(&input));

    divan::main();
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_coord(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Route {
    Finished(BTreeSet<(isize, isize)>),
    Loop,
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct AocMap {
    obstacles: BTreeSet<(isize, isize)>,
    guard_position: (isize, isize),
    width: isize,
    height: isize,
}

fn get_guard_route(map: &AocMap, extra_obsticle: Option<&(isize, isize)>) -> Route {
    let mut guard_position = map.guard_position.clone();
    let mut guard_direction = Direction::Up;
    let mut visited = BTreeSet::new();
    let mut visited_with_direction = BTreeSet::new();

    loop {
        let (dx, dy) = guard_direction.to_coord();
        let new_position = (guard_position.0 + dx, guard_position.1 + dy);
        if new_position.0 < 0 || new_position.0 >= map.width || new_position.1 < 0 || new_position.1 >= map.height {
            break;
        }

        if map.obstacles.contains(&new_position) || extra_obsticle.map_or(false, |o| o == &new_position) {
            guard_direction = guard_direction.turn();
            continue;
        }

        guard_position = new_position;

        if !visited_with_direction.insert((guard_position, guard_direction)) {
            return Route::Loop;
        }

        visited.insert(guard_position);
    }

    Route::Finished(visited)
}

fn parse_map(input: &str) -> AocMap {
    let mut guard_position = (0, 0);
    let mut obstacles = BTreeSet::new();

    input
    .lines().enumerate()
    .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, cell)| {
        match cell {
            '^' => guard_position = (x as isize, y as isize),
            '#' => { obstacles.insert((x as isize, y as isize)); },
            _ => {},
        }
    }));

    AocMap {
        obstacles,
        guard_position,
        width: input.lines().next().unwrap().len() as isize,
        height: input.lines().count() as isize,
    }
}

pub fn part1(input: &str) -> u32 {
    let map = parse_map(&input);

    match get_guard_route(&map, None) {
        Route::Finished(route) => route.len() as u32,
        _ => 0,
    }
}

pub fn part2(input: &str) -> u32 {
    let map = parse_map(&input);

    let visited = get_guard_route(&map, None);

    match visited {
        Route::Finished(route) => {
            let route = route.iter().collect::<Vec<_>>();
            route.into_par_iter()
            .filter(|(x, y)| {
                // Skip starting position
                if *x == map.guard_position.0 && *y == map.guard_position.1 {
                    return false;
                }

                get_guard_route(&map, Some(&(*x, *y))) == Route::Loop
            }).count() as u32
        },
        _ => 0,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#.to_string();
        let result = part1(&test);
        assert_eq!(result, 41)
    }

    #[test]
    fn test_part2() {
        let test = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#.to_string();
        let result = part2(&test);
        assert_eq!(result, 6)
    }

    #[divan::bench]
    fn bench_part1() {
       part1(divan::black_box(include_str!(
            "../../inputs/day6.txt",
        )));
    }

    #[divan::bench]
    fn bench_part2() {
        part2(divan::black_box(include_str!(
            "../../inputs/day6.txt",
        )));
    }
}