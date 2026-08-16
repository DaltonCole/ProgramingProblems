struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();

        if total % 4 != 0 {
            return false;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_partition_k_subsets(
            vec![4, 3, 2, 3, 5, 2, 1],
            4
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3));
    }
}
