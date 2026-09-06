use advent_of_code::day10_expand;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    // ALWAYS trim the AoC inputs!!!
    Some(
        (0..40)
            .fold(input.trim().to_owned(), |acc, _| day10_expand(&acc))
            .len() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // following AoC tradition, if you brute forced part 1 and part 2 just asks for _more_
    // then your part 1 solution won't complete any time soon...
    Some(
        (0..50)
            .fold(input.trim().to_owned(), |acc, _| day10_expand(&acc))
            .len() as u64,
    )
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
    #[case("1321312", "11131211131112")]
    fn test_part_one(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day10_expand(input), expected);
    }
}
