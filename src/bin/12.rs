use regex::Regex;
use serde_json::Value;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    Some(input.trim().lines().fold(0, |acc, line| {
        acc + re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .fold(0, |acc, n| acc + n)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed: Value = serde_json::from_str(input.trim()).unwrap();
    // dbg!(&parsed);
    // basically, need to recurse through this whole structure and ignore any object with a value "red"
    // which sounds tedious, skipping for now
    None
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
