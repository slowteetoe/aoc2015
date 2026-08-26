use md5::{Digest, Md5};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.trim();
    (1..=3_000_000).into_par_iter().find_first(|i| {
        // too many allocations, but roll with it for now...
        let s = Md5::digest(format!("{}{}", input, i).as_bytes());
        hex::encode(s).starts_with("00000")
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.trim();
    (1..=99_000_000).into_par_iter().find_first(|i| {
        // too many allocations, but roll with it for now...
        let s = Md5::digest(format!("{}{}", input, i).as_bytes());
        hex::encode(s).starts_with("000000")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", Some(609043))]
    #[case("pqrstuv", Some(1048970))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<u64>) {
        let result = part_one(&input);
        assert_eq!(result, expected);
    }
}
