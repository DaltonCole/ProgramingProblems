struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        return Self::n2_constant_space(&nums);
        //return Self::n2_end_early(&nums);
        //return Self::n2(&nums);
    }

    fn n2_constant_space(nums: &Vec<i32>) -> i32 {
        let mut majority = nums[0];
        let mut count = 1;

        for &num in nums.iter().skip(1) {
            if majority == num {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    count = 1;
                    majority = num;
                }
            }
        }
        majority
    }

    fn n2_end_early(nums: &Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, usize>::new();
        let half = (nums.len() + 1) / 2;

        for &num in nums {
            *map.entry(num).or_default() += 1;
            if map.get(&num).unwrap() >= &half {
                return num;
            }
        }

        *map.iter().max_by_key(|entry| entry.1).unwrap().0
    }

    fn n2(nums: &Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, usize>::new();

        for &num in nums {
            *map.entry(num).or_default() += 1;
        }

        *map.iter().max_by_key(|entry| entry.1).unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }

    #[test]
    fn test3() {
        assert_eq!(1, Solution::majority_element(vec![1, 1, 1, 1, 2, 2, 2]));
    }

    #[test]
    fn test4() {
        assert_eq!(
            1,
            Solution::majority_element(vec![1, 1, 1, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 2, 2, 2])
        );
    }
}
