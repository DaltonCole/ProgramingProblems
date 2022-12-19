struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.insert(0, 0);
        Self::helper(&cost[..])
    }

    fn helper(cost: &[i32]) -> i32 {
        match cost.len() {
            0 => 0,
            1 => cost[0],
            _ => cost[0] + std::cmp::min(Self::helper(&cost[1..]), Self::helper(&cost[2..])),
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
