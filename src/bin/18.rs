advent_of_code::solution!(18);

#[derive(Debug)]
pub struct Grid {
    cols: usize,
    rows: usize,
    data: Vec<u8>,
}

impl Grid {
    pub fn lit(&self) -> u64 {
        self.data.iter().filter(|v| **v == 1).count() as u64
    }

    pub fn step(&mut self) {
        let mut new_grid: Vec<_> = Vec::with_capacity(self.rows * self.cols);
        for (v, status) in self.data.iter().enumerate() {
            let (row, col) = self.to_row_col(v);
            let row = row as isize;
            let col = col as isize;
            let neighbor_count: usize = [
                // up
                self.value_at(row - 1, col),
                // up right
                self.value_at(row - 1, col + 1),
                // right
                self.value_at(row, col + 1),
                // down right
                self.value_at(row + 1, col + 1),
                // down
                self.value_at(row + 1, col),
                // down left
                self.value_at(row + 1, col - 1),
                // left
                self.value_at(row, col - 1),
                // up left
                self.value_at(row - 1, col - 1),
            ]
            .iter()
            .filter(|v| v.is_some() && v.unwrap() == 1)
            .map(|_| 1)
            .sum();
            // println!("{row} {col} -> {neighbor_count}");
            let new_val = match (status, neighbor_count) {
                (1, 2) | (1, 3) => 1,
                (0, 3) => 1,
                _ => 0,
            };
            new_grid.push(new_val);
        }
        self.data = new_grid;
    }

    fn to_row_col(&self, pos: usize) -> (usize, usize) {
        (pos / self.cols, pos.rem_euclid(self.cols))
    }

    pub fn value_at(&self, row: isize, col: isize) -> Option<u8> {
        if row < 0 || row >= self.rows as isize || col < 0 || col >= self.cols as isize {
            return None;
        }
        Some(self.data[(row * self.cols as isize + col) as usize])
    }

    fn illuminate_corners(&mut self) {
        self.data[0] = 1;
        self.data[self.cols - 1] = 1;
        self.data[self.rows * self.cols - self.cols] = 1;
        self.data[self.rows * self.cols - 1] = 1;
    }
}

pub fn parse_input(input: &str) -> Grid {
    let rows = input.trim().lines().count();
    let cols = input.trim().lines().nth(0).unwrap().len();

    let mut data = Vec::with_capacity(rows * cols);
    input.trim().lines().for_each(|line| {
        line.chars().for_each(|c| {
            data.push(match c {
                '#' => 1,
                _ => 0,
            });
        });
    });

    Grid {
        cols: cols,
        rows: rows,
        data: data,
    }
}

fn part_one_internal(step_count: usize, input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    // dbg!(&grid);
    for _ in 0..step_count {
        grid.step();
    }
    Some(grid.lit())
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_internal(100, input)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    grid.illuminate_corners();
    for _ in 0..100 {
        grid.step();
        grid.illuminate_corners();
    }
    Some(grid.lit())
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
