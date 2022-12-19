struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => std::cmp::max(
                Self::rob_helper(&nums[1..]),              // Try without the first house
                Self::rob_helper(&nums[..nums.len() - 1]), // Try without the last house
            ),
        }
    }

    pub fn rob_helper(nums: &[i32]) -> i32 {
        let mut size = nums.len();
        let mut mem = vec![0; size];

        for index in 0..size {
            size -= 1;
            match index {
                0 => mem[index] = nums[size],
                1 => {
                    mem[index] = std::cmp::max(nums[size], nums[size + 1]);
                }
                _ => mem[index] = std::cmp::max(nums[size] + mem[index - 2], mem[index - 1]),
            }
        }
        *mem.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    }

    #[test]
    fn test2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }
    #[test]
    fn test3() {
        assert_eq!(3, Solution::rob(vec![1, 2, 3]));
    }
    #[test]
    fn test4() {
        assert_eq!(30, Solution::rob(vec![10, 1, 1, 10, 1, 10, 1]));
    }
    #[test]
    fn test5() {
        assert_eq!(21, Solution::rob(vec![10, 1, 1, 10, 1, 10]));
    }

    #[test]
    fn test6() {
        assert_eq!(2, Solution::rob(vec![2]));
    }
}

// Better solution: https://leetcode.com/problems/house-robber-ii/discuss/1494213/Rust-0ms-2MB
