struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Tile {
    cost: i32,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(cost: i32, x: usize, y: usize) -> Tile {
        Tile { cost, x, y }
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        //.then(other.x.cmp(&self.x).reverse())
        //.then(other.y.cmp(&self.y).reverse())
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::dp(&grid)
        //Self::bfs(&grid)
    }

    fn dp(grid: &Vec<Vec<i32>>) -> i32 {
        let end_x = grid.len();
        let end_y = grid[0].len();

        let mut best_cost = vec![vec![i32::MAX; end_y]; end_x];
        best_cost[0][0] = grid[0][0];

        for i in 1..end_x {
            best_cost[i][0] = best_cost[i - 1][0] + grid[i][0];
        }
        for j in 1..end_y {
            best_cost[0][j] = best_cost[0][j - 1] + grid[0][j];
        }

        for i in 1..end_x {
            for j in 1..end_y {
                best_cost[i][j] =
                    std::cmp::min(best_cost[i][j - 1], best_cost[i - 1][j]) + grid[i][j]
            }
        }
        return *best_cost.last().unwrap().last().unwrap();
    }

    fn bfs(grid: &Vec<Vec<i32>>) -> i32 {
        let mut priority_queue = BinaryHeap::<Tile>::new();
        let end_x = grid.len() - 1;
        let end_y = grid[0].len() - 1;
        let end_val = grid[end_x][end_y];

        let mut best_cost = vec![vec![i32::MAX; end_y + 1]; end_x + 1];

        priority_queue.push(Tile::new(grid[0][0], 0, 0));

        while let Some(tile) = priority_queue.pop() {
            // Are we there?
            if tile.y == end_y && tile.x == end_x {
                return tile.cost;
            }
            // If we go down, are we there?
            if tile.y + 1 == end_y && tile.x == end_x {
                return tile.cost + end_val;
            }
            // If we can go right, are we there?
            if tile.x + 1 == end_x && tile.y == end_y {
                return tile.cost + end_val;
            }
            if best_cost[tile.x][tile.y] > tile.cost {
                best_cost[tile.x][tile.y] = tile.cost;
            }
            // Can we go down?
            if tile.y + 1 <= end_y
                && best_cost[tile.x][tile.y + 1] > tile.cost + grid[tile.x][tile.y + 1]
            {
                priority_queue.push(Tile::new(
                    tile.cost + grid[tile.x][tile.y + 1],
                    tile.x,
                    tile.y + 1,
                ));
            }
            // Can we go right?
            if tile.x + 1 <= end_x
                && best_cost[tile.x + 1][tile.y] > tile.cost + grid[tile.x + 1][tile.y]
            {
                priority_queue.push(Tile::new(
                    tile.cost + grid[tile.x + 1][tile.y],
                    tile.x + 1,
                    tile.y,
                ));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_vec(arr: &[&[i32]]) -> Vec<Vec<i32>> {
        let mut v = Vec::new();

        for a in arr {
            v.push(a.to_vec());
        }

        v
    }

    #[test]
    fn test1() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            12,
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(0, Solution::min_path_sum(vec![vec![0]]));
    }

    #[test]
    fn test4() {
        assert_eq!(
            12,
            Solution::min_path_sum(to_vec(&[&[1, 2, 3], &[4, 5, 6]]))
        );
    }
}
