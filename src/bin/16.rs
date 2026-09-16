use std::{cell::LazyCell, collections::BTreeMap};

use itertools::Itertools;

advent_of_code::solution!(16);

const COMPOUNDS: LazyCell<Vec<&str>> = LazyCell::new(|| {
    vec![
        "children",
        "cats",
        "samoyeds",
        "pomeranians",
        "akitas",
        "vizslas",
        "goldfish",
        "trees",
        "cars",
        "perfumes",
    ]
});

pub fn parse_line(input: &str) -> (u64, BTreeMap<String, u64>) {
    let (auntie, detected) = input.trim().split_once(":").unwrap();
    let auntie = auntie.replace("Sue ", "").parse::<u64>().unwrap();
    let mut map = BTreeMap::<String, u64>::new();
    detected.split(",").for_each(|m| {
        let (k, v) = m.split_once(": ").unwrap();
        map.insert(k.trim().to_owned(), v.parse::<u64>().unwrap());
    });
    (auntie, map)
}

pub fn target() -> BTreeMap<String, u64> {
    BTreeMap::from([
        ("children".to_owned(), 3),
        ("cats".to_owned(), 7),
        ("samoyeds".to_owned(), 2),
        ("pomeranians".to_owned(), 3),
        ("akitas".to_owned(), 0),
        ("vizslas".to_owned(), 0),
        ("goldfish".to_owned(), 5),
        ("trees".to_owned(), 3),
        ("cars".to_owned(), 2),
        ("perfumes".to_owned(), 1),
    ])
}

pub fn target_fingerprint() -> u64 {
    fingerprint(target())
}

// wanted to do a fingerprint, but that didn't work well for a partial match - easier to just check each element as we process
pub fn fingerprint(map: BTreeMap<String, u64>) -> u64 {
    COMPOUNDS.iter().enumerate().fold(0, |acc, (idx, key)| {
        if map.contains_key(*key) {
            let mut v = 1 << 5;
            // we have 10 dimensions with values up to 10 (1010) to encode, use 5th bit as a presence indictor
            v |= map.get(*key).unwrap() << 5 * idx;
            acc | v
        } else {
            acc
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let target = target();

    let fingerprints = input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .collect_vec();

    fingerprints
        .iter()
        .filter_map(|(aunt, m)| {
            if m.iter()
                .all(|(k, v)| target.contains_key(k) && target.get(k).unwrap() == v)
            {
                Some(*aunt)
            } else {
                None
            }
        })
        .nth(0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let fingerprints = input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .collect_vec();

    fingerprints
        .iter()
        // more tedious, but easier to hardcode the entire test
        .filter_map(|(aunt, m)| {
            if (m.get(&"children".to_owned()).is_none()
                || m.get(&"children".to_owned()).is_some_and(|v| *v == 3))
                && (m.get(&"cats".to_owned()).is_none()
                    || m.get(&"cats".to_owned()).is_some_and(|v| *v > 7))
                && (m.get(&"samoyeds".to_owned()).is_none()
                    || m.get(&"samoyeds".to_owned()).is_some_and(|v| *v == 2))
                && (m.get(&"pomeranians".to_owned()).is_none()
                    || m.get(&"pomeranians".to_owned()).is_some_and(|v| *v < 3))
                && (m.get(&"akitas".to_owned()).is_none()
                    || m.get(&"akitas".to_owned()).is_some_and(|v| *v == 0))
                && (m.get(&"vizslas".to_owned()).is_none()
                    || m.get(&"vizslas".to_owned()).is_some_and(|v| *v == 0))
                && (m.get(&"goldfish".to_owned()).is_none()
                    || m.get(&"goldfish".to_owned()).is_some_and(|v| *v < 5))
                && (m.get(&"trees".to_owned()).is_none()
                    || m.get(&"trees".to_owned()).is_some_and(|v| *v > 3))
                && (m.get(&"cars".to_owned()).is_none()
                    || m.get(&"cars".to_owned()).is_some_and(|v| *v == 2))
                && (m.get(&"perfumes".to_owned()).is_none()
                    || m.get(&"perfumes".to_owned()).is_some_and(|v| *v == 1))
            {
                Some(*aunt)
            } else {
                None
            }
        })
        .nth(0)
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
