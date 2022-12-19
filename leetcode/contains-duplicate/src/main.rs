struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for num in nums {
            match set.contains(&num) {
                true => {
                    return true;
                }
                false => {
                    set.insert(num);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }
}
