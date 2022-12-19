struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if nums.len() < 3 {
            return result;
        }

        nums.sort();

        #[cfg(test)]
        println!("{nums:?}");

        let mut current = 0;
        while current < nums.len() - 2 {
            let mut left = current + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[current] + nums[left] + nums[right];

                #[cfg(test)]
                println!(
                    "{} + {} + {} = {}",
                    nums[current], nums[left], nums[right], sum
                );

                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    let mut v = vec![nums[current], nums[left], nums[right]];

                    #[cfg(test)]
                    v.sort();

                    result.push(v);

                    // Skip duplicate numbers
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while right > left && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }

            // Skip duplicate numbers
            while current < nums.len() - 2 && nums[current] == nums[current + 1] {
                current += 1;
            }
            current += 1;
        }

        result
    }

    pub fn slow_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut set = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            set.insert(num, i);
        }

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let sum = -(nums[i] + nums[j]);
                if let Some(&x) = set.get(&sum) {
                    if x != i && x != j {
                        let mut v = vec![nums[i], nums[j], sum];
                        v.sort();
                        result.push(v);
                    }
                }
            }
        }

        result.sort();
        result.dedup();

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        let mut answer = Solution::three_sum(vec![]);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test1() {
        let mut answer = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        answer.sort();
        assert_eq!(answer, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test2() {
        let mut answer = Solution::three_sum(vec![0, 1, 1]);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test3() {
        let mut answer = Solution::three_sum(vec![0, 0, 0]);
        answer.sort();
        assert_eq!(answer, vec![vec![0, 0, 0]]);
    }
}
