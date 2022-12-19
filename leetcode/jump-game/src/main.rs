struct Solution;

// Greedy
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_distance = nums[0];

        for i in 1..nums.len() {
            if max_distance == 0 {
                return false;
            }
            max_distance = std::cmp::max(max_distance - 1, nums[i]);
        }

        true
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn test4() {
        assert!(Solution::can_jump(vec![3, 2, 1, 0]));
    }

    #[test]
    fn test5() {
        assert!(Solution::can_jump(vec![4, 2, 0, 0, 1, 1, 4, 4, 4, 0, 4, 0]));
    }

    #[test]
    fn test6() {
        assert!(!Solution::can_jump(vec![0, 1]));
    }
}
