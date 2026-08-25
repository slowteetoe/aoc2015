use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let (hs, _) = input.chars().fold((HashSet::new(), (0, 0)), |mut acc, c| {
        if acc.0.is_empty() {
            acc.0.insert(acc.1);
        }
        let (mut hs, curr_house) = acc;
        let house = match c {
            '^' => (curr_house.0, curr_house.1 - 1),
            '>' => (curr_house.0 + 1, curr_house.1),
            'v' => (curr_house.0, curr_house.1 + 1),
            '<' => (curr_house.0 - 1, curr_house.1),
            _ => unreachable!("invalid direction"),
        };
        hs.insert(house);
        (hs, house)
    });
    Some(hs.iter().count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (hs, _) = input.chars().enumerate().fold(
        (HashSet::new(), [(0, 0), (0, 0)]),
        |(mut hs, mut santas), (idx, c)| {
            if hs.is_empty() {
                hs.insert((0, 0));
            }
            let which = idx % 2;
            let curr_house = santas[which];
            let next_house = match c {
                '^' => (curr_house.0, curr_house.1 - 1),
                '>' => (curr_house.0 + 1, curr_house.1),
                'v' => (curr_house.0, curr_house.1 + 1),
                '<' => (curr_house.0 - 1, curr_house.1),
                _ => unreachable!("invalid direction"),
            };
            hs.insert(next_house);
            santas[which] = next_house;
            (hs, santas)
        },
    );
    Some(hs.iter().count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part_one(#[case] input: &str, #[case] expected: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_part_two(#[case] input: &str, #[case] expected: u64) {
        let result = part_two(&input);
        assert_eq!(result, Some(expected));
    }
}
