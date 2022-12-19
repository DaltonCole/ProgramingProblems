struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        Self::helper(&nums[..], &mut map)
    }

    fn helper(cost: &[i32], map: &mut HashMap<usize, i32>) -> i32 {
        match map.get(&cost.len()) {
            Some(num) => *num,
            None => {
                let num = match cost.len() {
                    0 => 0,
                    1 => cost[0],
                    2 => std::cmp::max(cost[0], cost[1]),
                    _ => std::cmp::max(
                        cost[0] + Self::helper(&cost[2..], map),
                        Self::helper(&cost[1..], map),
                    ),
                };
                map.insert(cost.len(), num);
                num
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }

    #[test]
    fn test3() {
        assert_eq!(30, Solution::rob(vec![10, 1, 1, 10, 1, 10, 1]));
    }
}
