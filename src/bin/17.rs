use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(17);

// Needed a way to differentiate buckets (e.g. there are two buckets with 5)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Bucket(i32, usize);

fn find_min_container_combinations(
    target: i32,
    containers: &[i32],
    index: usize,
    current_count: usize,
    min_containers: &mut usize,
    combinations_count: &mut usize,
) {
    if target == 0 {
        match current_count.cmp(min_containers) {
            Ordering::Less => {
                // Found a new strictly smaller container count
                *min_containers = current_count;
                *combinations_count = 1; // Reset count for this new minimum
            }
            Ordering::Equal => {
                // Found another combination matching the current minimum
                *combinations_count += 1;
            }
            Ordering::Greater => {
                // This combination uses too many containers; ignore it
            }
        }
        return;
    }

    if target < 0 || index == containers.len() {
        return;
    }

    // If we've already used more or equal containers than our current minimum,
    // there's no point exploring further down the inclusion branch.
    // We can still try to skip items (exclude), but we can't add more
    if current_count >= *min_containers {
        find_min_container_combinations(
            target,
            containers,
            index + 1,
            current_count,
            min_containers,
            combinations_count,
        );
        return;
    }

    // Include the current container (increment current_count by 1)
    find_min_container_combinations(
        target - containers[index],
        containers,
        index + 1,
        current_count + 1,
        min_containers,
        combinations_count,
    );

    // Exclude the current container (current_count stays the same)
    find_min_container_combinations(
        target,
        containers,
        index + 1,
        current_count,
        min_containers,
        combinations_count,
    );
}

fn count_combinations(target: i32, containers: &[i32], index: usize) -> usize {
    if target == 0 {
        return 1; // Found a valid combination
    }
    if target < 0 || index == containers.len() {
        return 0; // Invalid or out of bounds
    }

    // Include the current container + exclude the current container
    let include = count_combinations(target - containers[index], containers, index + 1);
    let exclude = count_combinations(target, containers, index + 1);

    include + exclude
}

pub fn solutions_for(
    goal: i32,
    current_path: Vec<Bucket>,
    available_containers: &Vec<Bucket>,
    all_paths: &mut Vec<Vec<Bucket>>,
) {
    // println!("at {current_path:?}, avail={available_containers:?}, found={all_paths:?}");
    let sum = current_path.iter().map(|b| b.0).sum::<i32>();
    // dbg!(&sum);
    if sum == goal {
        // println!("found viable path at [{current_path:?}]");
        all_paths.push(current_path.clone());
        return;
    } else if sum > goal {
        // println!("too high, just returning");
        return;
    } else {
        // println!("recursing from {current_path:?}...");
        for c in available_containers
            .clone()
            .iter()
            .filter(|c| sum + c.0 <= goal)
        {
            let mut current_path = current_path.clone();
            current_path.push(*c);
            let mut ac = available_containers.clone();
            ac.remove(
                available_containers
                    .iter()
                    .find_position(|this_container| *this_container == c)
                    .unwrap()
                    .0,
            );
            // println!(
            //     "\tadded {c:?} to the current_path {current_path:?}, available containers are {ac:?}"
            // );
            solutions_for(goal, current_path, &ac, all_paths);
        }
    }
}

fn parse_input(input: &str) -> Vec<Bucket> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let cap = line.parse::<i32>().unwrap();
            Bucket(cap, idx)
        })
        .sorted()
        .rev()
        .collect()
}

// well, part_one_internal comes up with the right answer...in 109 minutes
pub fn part_one(input: &str) -> Option<u64> {
    let goal = 150;
    let containers = parse_input(input).iter().map(|c| c.0).collect_vec();
    let result = count_combinations(goal, &containers, 0);
    Some(result as u64)
}

pub fn part_one_internal(goal: i32, containers: Vec<Bucket>) -> Option<u64> {
    let mut solutions = vec![];
    solutions_for(goal, vec![], &containers, &mut solutions);
    let solutions = solutions
        .iter()
        .map(|b| {
            let mut b = b.clone();
            b.sort();
            b
        })
        .unique()
        .collect_vec();
    // dbg!(&solutions);
    Some(solutions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let goal = 150;
    let containers = parse_input(input).iter().map(|c| c.0).collect_vec();
    let mut min_containers = usize::MAX;
    let mut combinations_count = 0;
    find_min_container_combinations(
        goal,
        &containers,
        0,
        0,
        &mut min_containers,
        &mut combinations_count,
    );
    println!("used {min_containers} for {combinations_count} combinations");
    Some(combinations_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let goal = 25;
        let containers = vec![
            Bucket(20, 1),
            Bucket(15, 1),
            Bucket(10, 1),
            Bucket(5, 1),
            Bucket(5, 2),
        ];
        let result = part_one_internal(goal, containers);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
