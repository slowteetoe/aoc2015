use std::str::FromStr;

use itertools::Itertools;

use crate::Command::{Toggle, TurnOn};

advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
enum Command {
    TurnOn((u16, u16), (u16, u16)),
    TurnOff((u16, u16), (u16, u16)),
    Toggle((u16, u16), (u16, u16)),
}

#[derive(Debug, PartialEq)]
struct Instruction {
    command: Command,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // toggle turn off turn on
        // toggle 461,550 through 564,900
        // 461,550 through 564,900
        let mut parts: Vec<&str> = s.split(" ").collect();
        parts.reverse();
        let (to_x, to_y) = parts[0].trim().split_once(",").unwrap();
        let (from_x, from_y) = parts[2].trim().split_once(",").unwrap();

        Ok(match parts[3] {
            "off" => Command::TurnOff(
                (
                    from_x.parse::<u16>().unwrap(),
                    from_y.parse::<u16>().unwrap(),
                ),
                (to_x.parse::<u16>().unwrap(), to_y.parse::<u16>().unwrap()),
            ),
            "on" => TurnOn(
                (
                    from_x.parse::<u16>().unwrap(),
                    from_y.parse::<u16>().unwrap(),
                ),
                (to_x.parse::<u16>().unwrap(), to_y.parse::<u16>().unwrap()),
            ),
            "toggle" => Toggle(
                (
                    from_x.parse::<u16>().unwrap(),
                    from_y.parse::<u16>().unwrap(),
                ),
                (to_x.parse::<u16>().unwrap(), to_y.parse::<u16>().unwrap()),
            ),
            _ => panic!("missed something {}", parts[3]),
        })
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| Instruction {
            command: Command::from_str(line).unwrap(),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lights = [0u8; 1_000_000];

    let instructions = parse_instructions(input);
    instructions.iter().for_each(|inst| match inst.command {
        TurnOn(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] = 1;
            }
        }
        Command::TurnOff(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] = 0;
            }
        }
        Toggle(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] = if lights[coord] == 1 { 0 } else { 1 };
            }
        }
    });
    Some(lights.iter().filter(|v| **v == 1).count() as u64)
}

fn to_array_coords(from: (u16, u16), to: (u16, u16)) -> Vec<usize> {
    let mut positions = Vec::new();
    for row in from.1..=to.1 {
        for col in from.0..=to.0 {
            positions.push(row as usize * 1000usize + col as usize);
        }
    }
    positions
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lights = [0u8; 1_000_000];

    let instructions = parse_instructions(input);
    instructions.iter().for_each(|inst| match inst.command {
        TurnOn(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] += 1;
            }
        }
        Command::TurnOff(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] = if lights[coord] > 0 {
                    lights[coord] - 1
                } else {
                    0
                }
            }
        }
        Toggle(from, to) => {
            for coord in to_array_coords(from, to) {
                lights[coord] += 2;
            }
        }
    });
    let total: u64 = lights.iter().fold(0, |acc, n| acc + *n as u64);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500", vec![Instruction{command: Command::TurnOn((0,0),(999,999))}, Instruction{command: Command::Toggle((0,0),(999,0))}, Instruction{command:Command::TurnOff((499,499),(500,500))}] )]
    fn test_parsing_instructions(#[case] input: &str, #[case] expected: Vec<Instruction>) {
        let result = parse_instructions(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case((0,0),(999,999), 1_000_000)]
    #[case((0,0),(999,0), 1_000)]
    #[case((499,499),(500,500), 4)]
    fn test_coords(#[case] from: (u16, u16), #[case] to: (u16, u16), #[case] expected_count: u64) {
        let coords = to_array_coords(from, to);
        let result = coords.iter().count() as u64;
        if result < 10 {
            dbg!(&coords);
        }
        assert_eq!(result, expected_count)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
