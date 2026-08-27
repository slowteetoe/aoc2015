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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
    #[case("jchzalrnumimnmhp")]
    #[case("haegwjzuvuyypxyu")]
    #[case("dvszwmarrgswjxmb")]
    fn test_naughty_lines(#[case] input: &str) {
        assert_eq!(is_nice_line(input), false)
    }
}
