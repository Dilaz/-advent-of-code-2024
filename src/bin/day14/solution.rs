#[path = "../../utils.rs"]
pub mod utils;
use std::collections::HashSet;

use glam::IVec2;
use nom::{bytes::complete::tag, character::complete::{self, newline, space1}, multi::separated_list1, sequence::{preceded, separated_pair}, IResult};
pub use utils::Solution;
use miette::Result;
use itertools::Itertools;

pub struct Day14;

const PART1_TIME: i32 = 100;

#[derive(Debug)]
pub struct Robot {
    location: IVec2,
    velocity: IVec2,
}

fn draw_map(robots: &[Robot], map_size: &IVec2) {
    let coords = robots.iter().map(|robot| robot.location).collect::<HashSet<_>>();
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            if coords.contains(&IVec2::new(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_safety_factor(robots: &[Robot], map_size: &IVec2) -> u32 {
    let mid = map_size / 2;

    let mut squadrons = [0, 0, 0, 0];
    robots.iter().for_each(|robot| {
        let location = robot.location;
        if location.x < mid.x && location.y < mid.y {
            squadrons[0] += 1;
        } else if location.x > mid.x && location.y < mid.y {
            squadrons[1] += 1;
        } else if location.x < mid.x && location.y > mid.y {
            squadrons[2] += 1;
        } else if location.x > mid.x && location.y > mid.y {
            squadrons[3] += 1;
        }
    });

    squadrons.into_iter().product()
}

pub fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    let res = separated_list1(newline, separated_pair(
        preceded(tag("p="), separated_pair(complete::u32, tag(","), complete::u32)),
        space1,
            preceded(tag("v="),
        separated_pair(complete::i32, tag(","), complete::i32)
    )
    ))(input)?;

    Ok((input, res.1.iter().map(|line| Robot {
        location: IVec2::new(line.0.0 as i32, line.0.1 as i32),
        velocity: IVec2::new(line.1.0, line.1.1),
    }).collect()))
}

impl Solution<u32> for Day14 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        let mut robots = parse(input).unwrap().1;
        let map_size = if cfg!(test) { IVec2::new(11, 7) } else { IVec2::new(101, 103) };
        let mul_vec = IVec2::splat(PART1_TIME);

        robots.iter_mut().for_each(|robot| {
            let mut location = robot.location;
            let movement = robot.velocity * mul_vec;
            location += movement;
            robot.location = location.rem_euclid(map_size);
        });

        Ok(get_safety_factor(&robots, &map_size))
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        let mut robots = parse(input).unwrap().1;
        let map_size = IVec2::new(101, 103);
        let mut counter = 1;

        loop {
            robots.iter_mut().for_each(|robot| {
                let mut location = robot.location;
                let movement = robot.velocity;
                location += movement;
                robot.location = location.rem_euclid(map_size);
            });

            // Let's assume, that when robots are doing the easter egg, no robots are on top of each other
            if robots.iter().map(|r| r.location).all_unique() {
                if !cfg!(test) {
                    draw_map(&robots, &map_size);
                }
                break;
            }

            counter += 1;
        }

        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day14, Solution};

    #[test]
    fn test_part1() {
        let test = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let result = Day14::part1(test);
        assert_eq!(result.unwrap(), 12)
    }
}
