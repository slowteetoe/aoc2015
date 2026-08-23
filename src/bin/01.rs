use std::ops::ControlFlow;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .chars()
            .fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 }),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    match input.char_indices().try_fold(0, |acc, c| {
        let new_floor = if c.1 == '(' { acc + 1 } else { acc - 1 };
        if new_floor == -1 {
            ControlFlow::Break(c.0 as i64)
        } else {
            ControlFlow::Continue(new_floor)
        }
    }) {
        ControlFlow::Break(result) => Some(result + 1),
        ControlFlow::Continue(_) => {
            println!("didn't find the solution!");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
