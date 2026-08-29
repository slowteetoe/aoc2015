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
    // oops, originally made this more difficult (and wrong) by not allowing 3 of the same char... e.g. aaa
    for (l, _boundary, r) in input
        .trim()
        .chars()
        .map(Some)
        .chain(std::iter::once(None))
        .chain(std::iter::once(None))
        .tuple_windows()
    {
        if l == r {
            separated = true;
        }
    }

    // figure out if there are any repeated pairs
    // basically, does input[i+2..] contain input[i] + input[i+1]?
    let mut repeated_pairs = false;
    let chars = input.trim().chars().collect_vec();
    'outer: for i in 0..chars.len() - 3 {
        // no way for last two chars to match, so avoid indexing problems
        for n in i + 2..chars.len() - 1 {
            if chars[i] == chars[n] && chars[i + 1] == chars[n + 1] {
                repeated_pairs = true;
                break 'outer;
            }
        }
    }

    separated && repeated_pairs
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| (line, is_nice_part2(line)))
            // .inspect(|f| println!("{:?}", f))
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
