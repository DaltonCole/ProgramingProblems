struct Solution;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        // Gold starting from the best spot
        let mut max_gold = 0;
        // Rows and cols
        let rows = grid.len();
        let cols = grid[0].len();
        // Remember where we've traveled
        let mut traveled = vec![vec![false; cols]; rows];

        // Test every starting location
        for row in 0..rows {
            for col in 0..cols {
                max_gold = std::cmp::max(
                    max_gold,
                    Self::recursive_gold(&grid, row as i32, col as i32, &mut traveled),
                );
            }
        }

        max_gold
    }

    fn recursive_gold(
        grid: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        traveled: &mut Vec<Vec<bool>>,
    ) -> i32 {
        // If we are out of bounds, there is no gold, or we've already been here, give up
        if row < 0
            || row >= grid.len() as i32
            || col < 0
            || col >= grid[0].len() as i32
            || grid[row as usize][col as usize] == 0
            || traveled[row as usize][col as usize] == true
        {
            return 0;
        }

        // Pick up gold for this square
        let mut max_gold = grid[row as usize][col as usize];
        traveled[row as usize][col as usize] = true;

        // Test going each direction
        let go_up = Self::recursive_gold(grid, row - 1, col, traveled) + max_gold;
        let go_down = Self::recursive_gold(grid, row + 1, col, traveled) + max_gold;
        let go_left = Self::recursive_gold(grid, row, col - 1, traveled) + max_gold;
        let go_right = Self::recursive_gold(grid, row, col + 1, traveled) + max_gold;

        // Find the best path
        max_gold = go_up;
        max_gold = std::cmp::max(go_down, max_gold);
        max_gold = std::cmp::max(go_left, max_gold);
        max_gold = std::cmp::max(go_right, max_gold);

        // Undo our travels so we can test this square in the future
        traveled[row as usize][col as usize] = false;

        max_gold
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
