struct Solution;

use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
            Direction::Center => write!(f, "⥁"),
        }
    }
}

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid_flow = grid.clone();
        let rows = grid.len();
        let cols = grid[0].len();
        let mut direction_grid = vec![vec![Direction::Center; cols]; rows];

        while Self::update_grid_flow(&grid, &mut grid_flow, &mut direction_grid) {}

        #[cfg(test)]
        for row in &grid {
            println!("{:?}", row);
        }
        #[cfg(test)]
        println!("");
        #[cfg(test)]
        for row in &direction_grid {
            println!("{:?}", row);
        }
        #[cfg(test)]
        println!("");
        #[cfg(test)]
        for row in &grid_flow {
            println!("{:?}", row);
        }
        *grid_flow
            .iter()
            .map(|x| x.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0)
    }

    fn update_grid_flow(
        grid: &Vec<Vec<i32>>,
        grid_flow: &mut Vec<Vec<i32>>,
        direction_grid: &mut Vec<Vec<Direction>>,
    ) -> bool {
        let mut updated = false;
        for row in 0..grid_flow.len() {
            for col in 0..grid_flow[0].len() {
                let (max, dir) = Self::max_nearby(&grid_flow, &direction_grid, row, col);
                if dir != Direction::Center {
                    grid_flow[row][col] = grid[row][col] + max;
                    direction_grid[row][col] = dir;
                    updated = true;
                }
            }
        }

        updated
    }

    fn max_nearby(
        grid_flow: &Vec<Vec<i32>>,
        direction_grid: &Vec<Vec<Direction>>,
        row: usize,
        col: usize,
    ) -> (i32, Direction) {
        let mut max = grid_flow[row][col];
        let mut dir = Direction::Center;
        if max == 0 {
            return (0, Direction::Center);
        }

        // Up
        if row != 0 {
            if grid_flow[row - 1][col] > max
                && !Self::cycle_protection(
                    direction_grid,
                    row - 1,
                    col,
                    &mut HashSet::from([(row, col)]),
                )
            {
                max = grid_flow[row - 1][col];
                dir = Direction::Up;
            }
        }
        // Down
        if row < grid_flow.len() - 1 {
            if grid_flow[row + 1][col] > max
                && !Self::cycle_protection(
                    direction_grid,
                    row + 1,
                    col,
                    &mut HashSet::from([(row, col)]),
                )
            {
                max = grid_flow[row + 1][col];
                dir = Direction::Down;
            }
        }
        // Left
        if col != 0 {
            if grid_flow[row][col - 1] > max
                && !Self::cycle_protection(
                    direction_grid,
                    row,
                    col - 1,
                    &mut HashSet::from([(row, col)]),
                )
            {
                max = grid_flow[row][col - 1];
                dir = Direction::Left;
            }
        }
        // Right
        if col < grid_flow[0].len() - 1 {
            if grid_flow[row][col + 1] > max
                && !Self::cycle_protection(
                    direction_grid,
                    row,
                    col + 1,
                    &mut HashSet::from([(row, col)]),
                )
            {
                max = grid_flow[row][col + 1];
                dir = Direction::Right;
            }
        }

        (max, dir)
    }

    fn cycle_protection(
        direction_grid: &Vec<Vec<Direction>>,
        mut current_row: usize,
        mut current_col: usize,
        previous_points: &mut HashSet<(usize, usize)>,
    ) -> bool {
        match direction_grid[current_row][current_col] {
            Direction::Center => return false,
            Direction::Up => {
                current_row -= 1;
            }
            Direction::Down => {
                current_row += 1;
            }
            Direction::Left => {
                current_col -= 1;
            }
            Direction::Right => {
                current_col += 1;
            }
        }
        let new_point = (current_row, current_col);

        if previous_points.contains(&new_point) {
            return true;
        } else {
            previous_points.insert(new_point);
            Self::cycle_protection(&direction_grid, current_col, current_col, previous_points)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]];
        assert_eq!(24, Solution::get_maximum_gold(input));
    }

    #[test]
    fn test2() {
        let input = vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20],
        ];
        assert_eq!(28, Solution::get_maximum_gold(input));
    }

    #[test]
    fn test3() {
        let input = vec![
            vec![1, 0, 7, 0, 0, 0],
            vec![2, 0, 6, 0, 1, 0],
            vec![3, 5, 6, 7, 4, 2],
            vec![4, 3, 1, 0, 2, 0],
            vec![3, 0, 5, 0, 20, 0],
        ];
        assert_eq!(60, Solution::get_maximum_gold(input));
    }
}
