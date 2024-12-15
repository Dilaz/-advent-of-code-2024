#[path = "../../utils.rs"]
pub mod utils;

use glam::{DMat2, U64Vec2, UVec2};
use nom::{bytes::complete::tag, character::complete::{self, newline}, multi::separated_list1, sequence::{pair, preceded, terminated, tuple}, IResult};
pub use utils::Solution;
use miette::Result;
pub struct Day13;

const A_COST: u64 = 3;
const B_COST: u64 = 1;

const PART_2_BONUS: u64 = 10000000000000;

#[derive(Debug, PartialEq)]
pub struct Game {
    button_a: UVec2,
    button_b: UVec2,
    prize: UVec2,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, tuple) = tuple((
        terminated(pair(
            preceded(tag("Button A: X+"), complete::u64),
     preceded(tag(", Y+"), complete::u64)
        ), newline),
        terminated(pair(
            preceded(tag("Button B: X+"), complete::u64),
     preceded(tag(", Y+"), complete::u64)
        ), newline),
        pair(
            preceded(tag("Prize: X="), complete::u64),
     preceded(tag(", Y="), complete::u64)
        ),
    ))(input)?;

    let game = Game {
        button_a: UVec2::new(tuple.0.0 as u32, tuple.0.1 as u32),
        button_b: UVec2::new(tuple.1.0 as u32, tuple.1.1 as u32),
        prize: UVec2::new(tuple.2.0 as u32, tuple.2.1 as u32),
    };

    Ok((input, game))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(pair(newline, newline), parse_game)(input)
} 

impl Solution<u64> for Day13 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u64> {
        let games = parse(input).unwrap().1;

        Ok(games.into_iter().map(|game| -> u64 {
            let button_a = game.button_a;
            let button_b = game.button_b;
            let mut current = UVec2::ZERO;
            let mut solutions = vec![];

            for i in 0..=100u64 {
                if current.x > game.prize.x || current.y > game.prize.y {
                    break;
                }
                let distance = game.prize - current;
                if distance.x % button_a.x == 0 && distance.y % button_a.y == 0 && distance.y / button_a.y == distance.x / button_a.x {
                    solutions.push(i * B_COST + (distance.x / button_a.x) as u64 * A_COST);
                }

                current += button_b;
            }

            if solutions.is_empty() {
                0
            } else {
                solutions.into_iter().min().unwrap()
            }
        }).sum())
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u64> {
        let games = parse(input).unwrap().1;
        Ok(games.into_iter().map(|game| {
            let prize = U64Vec2::new(game.prize.x as u64 + PART_2_BONUS, game.prize.y as u64 + PART_2_BONUS);

            let matrix = DMat2::from_cols_array(&[
                game.button_a.x as f64,
                game.button_a.y as f64,
                game.button_b.x as f64,
                game.button_b.y as f64
            ]);
            let det = matrix.determinant();

            if det as i64 == 0 {
                return 0;
            }

            let mat_x = DMat2::from_cols_array(&[
                prize.x as f64,
                prize.y as f64,
                game.button_b.x as f64,
                game.button_b.y as f64
            ]);
            let det_x = mat_x.determinant();
            
            let mat_y = DMat2::from_cols_array(&[
                game.button_a.x as f64,
                game.button_a.y as f64,
                prize.x as f64,
                prize.y as f64,
            ]);
            let det_y = mat_y.determinant();
                
            let y = det_y / det;
            let x = det_x / det;

            if x.trunc() != x || y.trunc() != y || x < 0f64 || y < 0f64 {
                return 0;
            }
            
            x as u64 * A_COST + y as u64 * B_COST
        }).sum())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use glam::UVec2;
    #[allow(unused_imports)]
    use super::{Day13, Solution, Game, parse};

    #[test]
    fn test_parse() {
        let test = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"#;
        let result = parse(test);
        assert_eq!(result.unwrap().1, vec![Game {
            button_a: UVec2::new(94, 34),
            button_b: UVec2::new(22, 67),
            prize: UVec2::new(8400, 5400),
        }])
    }

    #[test]
    fn test_part1() {
        let test = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let result = Day13::part1(test);
        assert_eq!(result.unwrap(), 480)
    }

    #[test]
    fn test_part2() {
        let test = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let result = Day13::part2(test);
        assert_eq!(result.unwrap(), 875318608908)
    }
}
