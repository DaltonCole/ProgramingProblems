struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut max_diff = 0;
        for i in 0..nums.len() / 2 {
            max_diff = std::cmp::max(max_diff, nums[nums.len() - i - 1] + nums[i]);
        }

        max_diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(7, Solution::min_pair_sum(vec![3, 5, 2, 3]));
    }

    #[test]
    fn test2() {
        assert_eq!(8, Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
    }

    #[test]
    fn test3() {
        assert_eq!(
            8,
            Solution::min_pair_sum(vec![4, 1, 5, 1, 2, 5, 1, 5, 5, 4])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(16, Solution::min_pair_sum(vec![1, 8, 8, 8]));
    }

    #[test]
    fn test5() {
        assert_eq!(25, Solution::min_pair_sum(vec![1, 1, 3, 4, 4, 22, 23, 23]));
    }
}
