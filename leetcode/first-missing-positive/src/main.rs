struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for index in 0..nums.len() {
            while nums[index] > 0
                && nums[index] as usize <= nums.len()
                && nums[index] != nums[(nums[index] - 1) as usize]
            {
                let original = nums[index];
                let other = nums[original as usize - 1];
                nums[original as usize - 1] = original;
                nums[index] = other
            }
        }

        for (index, &num) in nums.iter().enumerate() {
            if (index + 1) as i32 != num {
                return (index + 1) as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    }
    #[test]
    fn test2() {
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    }
    #[test]
    fn test3() {
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
    #[test]
    fn test4() {
        assert_eq!(2, Solution::first_missing_positive(vec![1]));
    }
    #[test]
    fn test5() {
        assert_eq!(2, Solution::first_missing_positive(vec![1, 1]));
    }
}
