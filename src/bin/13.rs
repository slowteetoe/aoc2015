use std::sync::LazyLock;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone, PartialEq)]
enum Delta {
    Gain(u64),
    Lose(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Seating {
    who: String,
    delta: Delta,
    next_to: String,
}

const RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.").unwrap()
});

fn parse_line(input: &str) -> Option<Seating> {
    let (who, op, amount, next_to) = RE
        .captures(input.trim())
        .unwrap()
        .iter()
        .skip(1)
        .collect_tuple()
        .unwrap();

    let amount = amount.unwrap().as_str().parse::<u64>().unwrap();
    Some(Seating {
        who: who.unwrap().as_str().to_owned(),
        delta: match op.unwrap().as_str() {
            "gain" => Delta::Gain(amount),
            "lose" => Delta::Lose(amount),
            _ => unreachable!("could not determine delta"),
        },
        next_to: next_to.unwrap().as_str().to_owned(),
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let seatings = input
        .trim()
        .lines()
        .filter_map(|line| parse_line(line))
        .collect_vec();
    dbg!(&seatings);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use crate::Delta::Gain;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Alice would gain 54 happiness units by sitting next to Bob.", Some(Seating { who: "Alice".to_owned(), delta: Gain(54), next_to: "Bob".to_owned() }))]
    fn test_parsing(#[case] input: &str, #[case] expected: Option<Seating>) {
        assert_eq!(expected, parse_line(input));
    }

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
