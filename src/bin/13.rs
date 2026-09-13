use std::{collections::BTreeMap, sync::LazyLock};

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

fn get_seating_info(input: &str) -> (Vec<String>, Vec<Seating>) {
    let seatings = input
        .trim()
        .lines()
        .filter_map(|line| parse_line(line))
        .collect_vec();
    let actors = seatings
        .iter()
        .map(|s| s.who.clone())
        .unique()
        .collect_vec();
    (actors, seatings)
}

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
pub fn build_happiness_map(seatings: &Vec<Seating>) -> BTreeMap<(&str, &str), i64> {
    seatings.iter().fold(BTreeMap::new(), |mut acc, seating| {
        let units = match seating.delta {
            Delta::Gain(n) => n as i64,
            Delta::Lose(n) => (n as i64).checked_neg().unwrap(),
        };
        let t = (seating.who.as_str(), seating.next_to.as_str());
        acc.insert(t, units);
        acc
    })
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut actors, seatings) = get_seating_info(input);
    let happiness_map = build_happiness_map(&seatings);

    // now figure out the happiness scores
    // with 8 people in the input, there's only 7! (5040) combinations - so we can just check them all
    let p1 = actors.remove(0);
    let l = actors.len();
    let permutations = actors
        .into_iter()
        .permutations(l)
        .map(|mut perm| {
            perm.push(p1.clone());
            perm
        })
        .collect_vec();

    Some(permutations.iter().fold(i64::MIN, |mut acc, perm| {
        let h = happiness_score(perm, &happiness_map);
        if h > acc {
            acc = h;
        }
        acc
    }))
}

// going to get sneaky with part 2 - we have to add ourselves, but since the pairing will always be apathetic (0 score)
// we'll just NOT insert into the map and default to 0 instead of unwrapping
fn happiness_score(arrangement: &Vec<String>, happiness_map: &BTreeMap<(&str, &str), i64>) -> i64 {
    let combos: Vec<(_, _)> = arrangement
        .iter()
        .chain(vec![arrangement.iter().nth(0).unwrap()])
        .tuple_windows()
        .collect_vec();

    combos.iter().fold(0, |mut acc, combo| {
        // remember, people have different happiness levels - Alice might like Bob, but Bob hates Alice
        acc += happiness_map
            .get(&(combo.0.as_str(), combo.1.as_str()))
            .unwrap_or_else(|| &0);
        acc += happiness_map
            .get(&(combo.1.as_str(), combo.0.as_str()))
            .unwrap_or_else(|| &0);
        acc
    })
}

pub fn part_two(input: &str) -> Option<i64> {
    let (actors, seatings) = get_seating_info(input);
    let happiness_map = build_happiness_map(&seatings);

    // now figure out the happiness scores
    // with 8 people in the input, there's only 7! (5040) combinations - so we can just check them all
    let p1 = "ME";
    let l = actors.len();
    let permutations = actors
        .into_iter()
        .permutations(l)
        .map(|mut perm| {
            perm.push(p1.to_owned());
            perm
        })
        .collect_vec();

    Some(permutations.iter().fold(i64::MIN, |mut acc, perm| {
        let h = happiness_score(perm, &happiness_map);
        if h > acc {
            acc = h;
        }
        acc
    }))
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
    fn test_happiness_scores() {
        let (actors, seatings) =
            get_seating_info(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(4, actors.len());
        assert_eq!(12, seatings.len()); // 12 rows of data in the example
        let happiness_map = build_happiness_map(&seatings);
        assert_eq!(12, happiness_map.len());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
