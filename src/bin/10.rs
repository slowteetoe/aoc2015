advent_of_code::solution!(10);

// lots of alloc and sloooow...
pub fn expand(input: &str) -> String {
    let mut output = "".into();
    let mut prev_c = input.chars().nth(0).unwrap();
    let mut count = 1;
    for c in input.chars().skip(1) {
        if c == prev_c {
            count += 1;
        } else {
            output = format!("{output}{count}{prev_c}");
            count = 1;
            prev_c = c;
        }
    }
    format!("{output}{count}{prev_c}")
}

pub fn part_one(input: &str) -> Option<u64> {
    // ALWAYS trim the AoC inputs!!!
    Some(
        (0..40)
            .fold(input.trim().to_owned(), |acc, _| expand(&acc))
            .len() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_part_one(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expand(input), expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
