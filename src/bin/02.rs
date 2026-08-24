use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

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
                let mut sides = BinaryHeap::new();
                sides.push(Reverse(l * w));
                sides.push(Reverse(w * h));
                sides.push(Reverse(h * l));
                let Reverse(smallest) = sides.pop().unwrap();
                let Reverse(second_smallest) = sides.pop().unwrap();
                let Reverse(biggest) = sides.pop().unwrap();

                2 * smallest + 2 * second_smallest + 2 * biggest + smallest
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (l, w, h): (u32, u32, u32) = line
                    .splitn(3, "x")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let mut sides = BinaryHeap::new();
                sides.push(Reverse(l));
                sides.push(Reverse(w));
                sides.push(Reverse(h));

                let Reverse(smallest) = sides.pop().unwrap();
                let Reverse(second_smallest) = sides.pop().unwrap();
                let Reverse(biggest) = sides.pop().unwrap();

                (2 * smallest) + (2 * second_smallest) + (smallest * second_smallest * biggest)
            })
            .sum(),
    )
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
        assert_eq!(result, Some(34 + 14));
    }
}
