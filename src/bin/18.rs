advent_of_code::solution!(18);

#[derive(Debug)]
pub struct Grid(Vec<Vec<bool>>);

impl Grid {
    pub fn step() {}
}

pub fn parse_input(input: &str) -> Grid {
    Grid(
        input
            .trim()
            .lines()
            .enumerate()
            .fold(vec![], |mut acc, line| {
                acc.push(line.1.chars().fold(vec![], |mut acc, c| {
                    acc.push(match c {
                        '#' => true,
                        _ => false,
                    });
                    acc
                }));
                acc
            }),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    dbg!(&grid);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
