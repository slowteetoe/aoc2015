use std::sync::LazyLock;

use regex::Regex;

advent_of_code::solution!(8);

fn parse_line(line: &str) -> (u64, u64) {
    let escaped = LazyLock::new(|| Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap());
    let escquote = LazyLock::new(|| Regex::new(r#"\\""#).unwrap());
    let doubleslash = LazyLock::new(|| Regex::new(r#"\\\\"#).unwrap());

    let tmp = escaped.replace_all(&line, "*");
    let tmp = doubleslash.replace_all(&tmp, "^");
    let tmp = escquote.replace_all(&tmp, "|");

    dbg!(&tmp);

    (line.len() as u64, (tmp.to_string().len() - 2) as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| (line, parse_line(line)))
            .inspect(|v| {
                println!("{:?}", &v);
            })
            .map(|(_, b)| b.0 - b.1)
            .sum(),
    )
    // supposedly, 1344 is not correct
    //
    // sed 's/\\\\/#/g; s/\\"/#/g; s/\\x[a-f0-9][a-f0-9]/###/g; s/\"/#/g; s/[^#]//g' 08.txt | grep -o . | wc -l
    // returns 1342 for some reason
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, (2,0))]
    #[case(r#""abc""#, (5,3))]
    #[case(r#""aaa\"aaa""#, (10,7))]
    #[case(r#""\x27""#, (6,1))]
    #[case(r#""\"\xe8\"ec\xeah\"qo\\g\"iuqxy\"e\"y\xe7xk\xc6d""#, (48,27))]
    #[case(r#""d\\gkbqo\\fwukyxab\"u""#, (23,18))]
    #[case(r#""\xc4""#, (6,1))]
    #[case(r#""rq\\\"mohnjdf\\xv\\hrnosdtmvxot""#, (33,27))]
    #[case(r#""v\xfb\"lgs\"kvjfywmut\x9cr""#, (28,18))]
    fn test_parse_line(#[case] input: &str, #[case] expected: (u64, u64)) {
        assert_eq!(parse_line(input), expected);
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
