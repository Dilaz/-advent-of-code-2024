#[path = "../../utils.rs"]
pub mod utils;
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{self, alpha1, newline}, multi::separated_list1, sequence::{delimited, pair, preceded, terminated}, IResult};
pub use utils::Solution;
use miette::Result;
pub struct Day15;

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid command"),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn run(&mut self, stop_if: Option<&Vec<u8>>) {
        let mut instruction_pointer = 0usize;
        loop {
            if instruction_pointer >= self.program.len() {
                break;
            }

            let current_instruction = self.program.get(instruction_pointer).unwrap();
            let instruction = Instruction::from_u8(*current_instruction);
            let literal = self.program.get(instruction_pointer + 1).unwrap();
            let combo = self.get_combo(*literal);
            match instruction {
                Instruction::Adv => {
                    let num = self.register_a;
                    let den = 2u32.pow(combo as u32) as u64;
                    let res = num / den;
                    self.register_a = res;
                },
                Instruction::Bxl => {
                    self.register_b ^= *literal as u64;
                },
                Instruction::Bst => {
                    self.register_b = combo % 8;
                },
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        instruction_pointer = combo as usize;
                        continue;
                    }
                },
                Instruction::Bxc => {
                    self.register_b ^= self.register_c;
                },
                Instruction::Out => {
                    self.output.push((combo % 8) as u8);
                },
                Instruction::Bdv => {
                    let num = self.register_a;
                    let den = 2u32.pow(combo as u32) as u64;
                    let res = num / den;
                    self.register_b = res;
                },
                Instruction::Cdv => {
                    let num = self.register_a;
                    let den = 2u32.pow(combo as u32) as u64;
                    let res = num / den;
                    self.register_c = res;
                },
            };

            if let Some(stop_if) = stop_if {
                if self.output != stop_if[0..self.output.len()] {
                    break;
                }
            }

            instruction_pointer += 2;
        }
    }

    fn get_combo(&self, combo: u8) -> u64 {
        match combo {
            0..=3 => combo as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo"),
        }
    }
}

fn parse_register(input: &str) -> IResult<&str, u64> {
    let (input, (_, register)) = terminated(pair(
        delimited(tag("Register "), alpha1, tag(": ")),
        complete::u64
    ), newline)(input)?;

    Ok((input, register))
}


fn parse(input: &str) -> IResult<&str, Computer> {
    let (input, register_a) = parse_register(input)?;
    let (input, register_b) = parse_register(input)?;
    let (input, register_c) = parse_register(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = preceded(tag("Program: "), separated_list1(tag(","), complete::u8))(input)?;

    Ok((input, Computer {
        register_a,
        register_b,
        register_c,
        program: instructions,
        output: vec![],
    }))
}

impl Solution<String> for Day15 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<String> {
        let (_, mut computer) = parse(input).unwrap();
        computer.run(None);

        dbg!(&computer);

        Ok(computer.output.iter().map(|i| i.to_string()).collect_vec().join(","))
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<String> {
        let (_, computer) = parse(input).unwrap();


        let mut i = 0;
        loop {
            let mut computer_copy = computer.clone();
            computer_copy.register_a = i;
            computer_copy.run(Some(&computer.program.clone()));

            if computer_copy.output == computer_copy.program {
                break;
            }

            if i % 1_000_000 == 0 {
                println!("{}", i);
            }

            i += 1;
        }

        Ok(i.to_string())
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day15, Solution, parse, Computer};

    #[test]
    fn test_parse() {
        let test = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        let (input, result) = parse(test).unwrap();
        assert_eq!(input, "");
        assert_eq!(result, Computer {
            register_a: 729,
            register_b: 0,
            register_c: 0,
            program: vec![0,1,5,4,3,0],
            output: vec![],
        })
    }

    #[test]
    fn test_part1() {
        let test = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        let result = Day15::part1(test);
        assert_eq!(result.unwrap(), "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn test_part2() {
        let test = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        let result = Day15::part2(test);
        assert_eq!(result.unwrap(), "117440")
    }
}
