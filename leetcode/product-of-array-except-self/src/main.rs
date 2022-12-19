struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        // Forward pass
        for (i, num) in nums.iter().skip(1).enumerate() {
            result
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::product_except_self(vec![]), vec![]);
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4, 0, 5, 6, 7, 0, 8, 9]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
