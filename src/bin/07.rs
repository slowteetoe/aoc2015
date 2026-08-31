use std::collections::BTreeMap;

use itertools::Itertools;

use crate::Operation::{
    AndRegisters, AndValue, CopyRegister, CopyValue, LShift, NotRegister, OrRegisters, OrValue,
    RShift,
};

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    CopyValue(u64, String),
    CopyRegister(String, String),
    AndRegisters(String, String, String),
    AndValue(u64, String, String),
    OrRegisters(String, String, String),
    OrValue(u64, String, String),
    NotRegister(String, String),
    LShift(String, u64, String),
    RShift(String, u64, String),
}

fn parse_operations(input: &str) -> Vec<Operation> {
    let ops = input
        .trim()
        .lines()
        .map(|line| {
            let (lhs, dest_register) = line.split_once(" -> ").unwrap();
            let parts = lhs.split(" ").collect_vec();
            let dest_register = dest_register.to_owned();
            match parts.len() {
                1 => {
                    // either <number> or <register>
                    if let Ok(num) = parts[0].parse::<u64>() {
                        CopyValue(num, dest_register)
                    } else {
                        CopyRegister(parts[0].to_owned(), dest_register)
                    }
                }
                2 => {
                    if parts[0] == "NOT" {
                        NotRegister(parts[1].to_owned(), dest_register)
                    } else {
                        unreachable!("missed a 2 part operation")
                    }
                }
                3 => match parts[1] {
                    "AND" => {
                        if let Ok(num) = parts[0].parse::<u64>() {
                            AndValue(num, parts[2].to_owned(), dest_register)
                        } else {
                            AndRegisters(parts[0].to_owned(), parts[2].to_owned(), dest_register)
                        }
                    }
                    "OR" => {
                        if let Ok(num) = parts[0].parse::<u64>() {
                            OrValue(num, parts[2].to_owned(), dest_register)
                        } else {
                            OrRegisters(parts[0].to_owned(), parts[2].to_owned(), dest_register)
                        }
                    }
                    "LSHIFT" => LShift(
                        parts[0].to_owned(),
                        parts[2].parse::<u64>().unwrap(),
                        dest_register,
                    ),
                    "RSHIFT" => RShift(
                        parts[0].to_owned(),
                        parts[2].parse::<u64>().unwrap(),
                        dest_register,
                    ),
                    _ => unreachable!("missed a 3 part operation"),
                },
                _ => unimplemented!("{line}"),
            }
        })
        .collect_vec();
    ops
}

fn process_operations(mut operations: Vec<Operation>, registers: &mut BTreeMap<String, u64>) {
    // loop over the operations, eventually they should all be resolved

    let mut remaining_ops = operations.len();
    while remaining_ops > 0 {
        let remaining = operations
            .into_iter()
            .filter_map(|op| match &op {
                CopyValue(n, dest) => {
                    registers.insert(dest.clone(), *n);
                    None
                }
                CopyRegister(src, dest) => {
                    if registers.contains_key(src) {
                        registers.insert(dest.clone(), *registers.get(src).unwrap());
                        None
                    } else {
                        Some(op)
                    }
                }
                AndRegisters(a, b, dest) => {
                    if registers.contains_key(a) && registers.contains_key(b) {
                        let a = *registers.get(a).unwrap();
                        let b = *registers.get(b).unwrap();
                        registers.insert(dest.to_owned(), a & b);
                        None
                    } else {
                        Some(op)
                    }
                }
                AndValue(a, b, dest) => {
                    if registers.contains_key(b) {
                        let b = *registers.get(b).unwrap();
                        registers.insert(dest.to_owned(), a & b);
                        None
                    } else {
                        Some(op)
                    }
                }
                OrRegisters(a, b, dest) => {
                    if registers.contains_key(a) && registers.contains_key(b) {
                        let a = *registers.get(a).unwrap();
                        let b = *registers.get(b).unwrap();
                        registers.insert(dest.to_owned(), a | b);
                        None
                    } else {
                        Some(op)
                    }
                }
                OrValue(a, b, dest) => {
                    if registers.contains_key(b) {
                        let b = *registers.get(b).unwrap();
                        registers.insert(dest.to_owned(), a | b);
                        None
                    } else {
                        Some(op)
                    }
                }
                NotRegister(src, dest) => {
                    if registers.contains_key(src) {
                        registers.insert(dest.to_owned(), !*registers.get(src).unwrap());
                        None
                    } else {
                        Some(op)
                    }
                }
                LShift(src, amt, dest) => {
                    if registers.contains_key(src) {
                        registers.insert(dest.to_owned(), registers.get(src).unwrap() << amt);
                        None
                    } else {
                        Some(op)
                    }
                }
                RShift(src, amt, dest) => {
                    if registers.contains_key(src) {
                        registers.insert(dest.to_owned(), registers.get(src).unwrap() >> amt);
                        None
                    } else {
                        Some(op)
                    }
                }
            })
            .collect_vec();
        remaining_ops = remaining.len();
        operations = remaining;
    }
}

// maybe a consuming multipass across the lines, backed by "registers" (btreemap)
pub fn part_one(input: &str) -> Option<u64> {
    let operations = parse_operations(input);
    let mut registers = BTreeMap::<String, u64>::new();
    process_operations(operations, &mut registers);

    Some(*registers.get("a").unwrap()) // 3176
}

pub fn part_two(input: &str) -> Option<u64> {
    // Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?
    let mut operations = parse_operations(input);
    operations.retain(|v| *v != CopyValue(44430, "b".to_owned()));

    let mut registers = BTreeMap::<String, u64>::new();
    registers.insert("b".to_owned(), 3176);
    process_operations(operations, &mut registers);

    Some(*registers.get("a").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
