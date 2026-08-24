use itertools::Itertools;

advent_of_code::solution!(2);

// 2*l*w + 2*w*h + 2*h*l
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (l, w, h): (u32, u32, u32) = line
                    .splitn(3, "x")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let sides = [l * w, w * h, h * l];
                let slack = sides.iter().min().unwrap();
                slack + sides.iter().sum::<u32>() * 2
            })
            .sum(),
    )
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
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
