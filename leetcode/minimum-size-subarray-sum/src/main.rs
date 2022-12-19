struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }
    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::min_sub_array_len(10, vec![]), 0);
    }
}
