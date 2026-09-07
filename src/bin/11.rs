use itertools::Itertools;

advent_of_code::solution!(11);

pub fn next_valid_password(input: Vec<u8>) -> Vec<u8> {
    let mut candidate = generate_next_password(&input);
    loop {
        if is_valid_password(&candidate) {
            // println!("found: {candidate:?}");
            return candidate;
        } else {
            candidate = generate_next_password(&candidate);
        }
    }
}

#[inline]
fn generate_next_password(input: &Vec<u8>) -> Vec<u8> {
    let z = 'z' as u8;
    let a = 'a' as u8;
    let mut input = input.clone();
    input.reverse(); // reverse to make carry-over simpler
    for idx in 0..input.len() {
        input[idx] += 1;
        if input[idx] > z {
            input[idx] = a;
        } else {
            break;
        }
    }
    input.reverse(); // reverse to get original order
    input.to_owned()
}

pub fn is_valid_password(input: &Vec<u8>) -> bool {
    let invalid_i = 'i' as u8;
    let invalid_o = 'o' as u8;
    let invalid_l = 'l' as u8;
    if input.contains(&invalid_i) || input.contains(&invalid_o) || input.contains(&invalid_l) {
        return false;
    }
    if let Some(_) = input
        .iter()
        .tuple_windows()
        .find(|(a, b, c)| **b == *a + 1 && **c == *a + 2)
    {
        // println!("{a}-{b}-{c}");
        // now check to see if there were 2 repeated pairs, non-overlapping
        let mut pairs = 0;
        let mut prev = ' ' as u8;
        for i in input {
            if *i == prev {
                // println!("->{prev}-{i}<-");
                pairs += 1;
                prev = ' ' as u8; // can't overlap, so the rightmost of a pair should not match the next ever
            } else {
                prev = *i;
            }
        }
        return pairs >= 2;
    }
    false
}

pub fn part_one(input: &str) -> Option<String> {
    // AGAIN, make sure to trim the input!
    let next_password = next_valid_password(input.trim().chars().map(|c| c as u8).collect_vec());
    Some(String::from_utf8(next_password).unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_password_validity(#[case] input: &str, #[case] is_valid: bool) {
        let input = input.chars().map(|c| c as u8).collect();
        assert_eq!(is_valid_password(&input), is_valid);
    }

    #[rstest]
    #[case("abcdefgh", "abcdefgi")]
    fn test_generating_next_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            generate_next_password(&input.chars().map(|c| c as u8).collect::<Vec<_>>()),
            expected.chars().map(|c| c as u8).collect::<Vec<_>>()
        )
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    fn test_next_valid_password(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            next_valid_password(input.chars().map(|c| c as u8).collect::<Vec<_>>()),
            expected.chars().map(|c| c as u8).collect::<Vec<_>>()
        )
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
