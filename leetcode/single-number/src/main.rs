struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for num in nums {
            if set.remove(&num) == false {
                set.insert(num);
            }
        }
        for num in set {
            return num;
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }
    #[test]
    fn test2() {
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
    #[test]
    fn test3() {
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}

// Beautiful Solution: https://leetcode.com/problems/single-number/discuss/998258/Rust%3A-fold-solution
