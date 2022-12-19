struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        //return Self::slow(&nums, target);
        return Self::fast(&mut nums, target);
    }

    fn fast(nums: &mut Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexes = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(x) = indexes.get(&(target - num)) {
                return vec![index as i32, *x];
            }
            indexes.insert(num, index as i32);
        }
        vec![]
    }

    fn slow(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j && nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn bad(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sort_nums = nums.clone();
        sort_nums.sort();

        let mut i = 0;
        let mut j = sort_nums.len() - 1;

        while i < j {
            let sum = sort_nums[i] + sort_nums[j];

            #[cfg(test)]
            println!("{} = {} + {}", sum, sort_nums[i], sort_nums[j]);

            if sum == target {
                for (index, num) in nums.iter().enumerate() {
                    if sort_nums[i] == *num {
                        i = index;
                        break;
                    }
                }
                for (index, num) in nums.iter().enumerate() {
                    if sort_nums[j] == *num && index != i {
                        j = index;
                        break;
                    }
                }
                return vec![i as i32, j as i32];
            } else {
                if sum > target {
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        let mut answer = Solution::two_sum(vec![2, 7, 11, 15], 9);
        answer.sort();
        assert_eq!(answer, vec![0, 1]);
    }

    #[test]
    fn test1() {
        let mut answer = Solution::two_sum(vec![3, 2, 4], 6);
        answer.sort();
        assert_eq!(answer, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let mut answer = Solution::two_sum(vec![3, 3], 6);
        answer.sort();
        assert_eq!(answer, vec![0, 1]);
    }
}
