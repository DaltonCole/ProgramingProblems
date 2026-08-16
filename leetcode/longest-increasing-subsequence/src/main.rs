struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(4, Solution::length_of_lis(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(4, Solution::length_of_lis(nums));
    }

    #[test]
    fn test3() {
        let nums = vec![7, 7, 7, 7, 7, 7];
        assert_eq!(1, Solution::length_of_lis(nums));
    }
}
