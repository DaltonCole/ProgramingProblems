struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        Self::helper(&cost[..], &mut map)
    }

    fn helper(cost: &[i32], map: &mut HashMap<usize, i32>) -> i32 {
        match map.get(&cost.len()) {
            Some(num) => *num,
            None => {
                let num = match cost.len() {
                    0 | 1 => 0,
                    2 => std::cmp::min(cost[0], cost[1]),
                    _ => std::cmp::min(
                        cost[0] + Self::helper(&cost[1..], map),
                        cost[1] + Self::helper(&cost[2..], map),
                    ),
                };
                println!("{} = {:?}", num, cost);
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
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn test2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
