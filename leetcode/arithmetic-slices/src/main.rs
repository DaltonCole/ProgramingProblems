struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut count = 0;

        for start_index in 0..(nums.len() - 2) {
            for end_index in (start_index + 2)..nums.len() {
                if Self::subslice(&nums[start_index..=end_index]) {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        count
    }

    // Checks if the last element in the slice follows the arithmetic slice.
    // We assume that the first n-1 elements have already been checked
    fn subslice(nums: &[i32]) -> bool {
        #[cfg(test)]
        println!("{nums:?}");

        if nums.len() < 3 {
            return false;
        }

        let diff = nums[1] - nums[0];
        let len = nums.len();

        nums[len - 1] - nums[len - 2] == diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3]), 1);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 7, -2]), 0);
    }
}
