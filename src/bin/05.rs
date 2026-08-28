use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| is_nice_line(line))
            .filter(|nice| *nice == true)
            .count() as u64,
    )
}

fn is_nice_line(line: &str) -> bool {
    let mut vowels = 0;
    let mut dubs = false;

    for (c, c1) in line
        .chars()
        // annoying, but we need to deal with how tuple windows work;
        // we need to check if the last char in an even len string is a vowel
        .map(Some)
        .chain(std::iter::once(None))
        .tuple_windows()
    {
        if let Some(c) = c {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowels += 1;
            }
            if c1.is_some() && c == c1.unwrap() {
                dubs = true;
            }
            match (c, c1) {
                ('a', Some('b')) | ('c', Some('d')) | ('p', Some('q')) | ('x', Some('y')) => {
                    return false;
                }
                _ => continue,
            }
        }
    }
    vowels >= 3 && dubs
}

fn is_nice_part2(input: &str) -> bool {
    let mut separated = false;
    for (l, boundary, r) in input
        .trim()
        .chars()
        .map(Some)
        .chain(std::iter::once(None))
        .chain(std::iter::once(None))
        .tuple_windows()
    {
        if l == r && l != boundary {
            println!("found {l:?}{boundary:?}{r:?} as separated");
            separated = true;
        }
    }

    let mut repeated_pairs = 0;
    let mut pairs = HashMap::new();
    for (l, r) in input.trim().chars().map(Some).chain(None).tuple_windows() {
        if let Some(r) = r {
            pairs
                .entry(format!("{}{r}", l.unwrap()))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
    let interesting = pairs.iter().filter(|(_, v)| **v > 1).collect_vec();
    dbg!(&interesting);

    for (c, count) in interesting {
        if *count >= 2 {
            repeated_pairs += 1;
            // check to make sure there's no overlap though - this is expensive, but should be rare
            if c.chars().nth(0) == c.chars().nth(1)
                && input.contains(&format!("{0}{0}{0}", c.chars().nth(0).unwrap(),))
                && !input.contains(&format!("{0}{0}{0}{0}", c.chars().nth(0).unwrap(),))
            {
                println!("disallowed {c} because it occurred with overlap in {input}");
                repeated_pairs -= 1;
            }
        }
    }
    dbg!(&separated, &repeated_pairs);
    separated && repeated_pairs >= 1
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| (line, is_nice_part2(line)))
            .inspect(|f| println!("{:?}", f))
            .filter(|nice| nice.1 == true)
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn")]
    #[case("aaa")]
    #[case("vxsluutrwskslnye")]
    fn test_nice_lines(#[case] input: &str) {
        assert_eq!(is_nice_line(input), true)
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb")]
    #[case("xxyxx")]
    fn test_nice_lines_part2(#[case] input: &str) {
        assert_eq!(is_nice_part2(input), true)
    }

    #[rstest]
    #[case("jchzalrnumimnmhp")]
    #[case("haegwjzuvuyypxyu")]
    #[case("dvszwmarrgswjxmb")]
    fn test_naughty_lines(#[case] input: &str) {
        assert_eq!(is_nice_line(input), false)
    }

    #[rstest]
    #[case("uurcxstgmygtbstg")]
    #[case("ieodomkazucvgmuy")]
    fn test_naughty_lines_part2(#[case] input: &str) {
        assert_eq!(is_nice_part2(input), false)
    }
}
