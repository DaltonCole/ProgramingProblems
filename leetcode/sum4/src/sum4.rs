pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //return Self::brute_force(&nums, target);
        //return Self::brute_force_binary_search(&mut nums, target);
        nums.sort();
        Self::n3(&nums[..], target as i64, 4).into_iter().collect()
    }

    fn n3(nums: &[i32], target: i64, k_sum: usize) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < k_sum || k_sum < 2 {
            return result;
        }

        // Remaining numbers cannot sum to target
        if (target / k_sum as i64) < nums[0] as i64
            || (target / k_sum as i64) > *nums.last().unwrap() as i64
        {
            return result;
        }

        if k_sum > 2 {
            let mut i = 0;
            while i < (nums.len() - k_sum + 1) {
                let difference = target - nums[i] as i64;
                let mut found = Self::n3(&nums[(i + 1)..], difference, k_sum - 1);
                for mut k_minus_1_sum in found.drain(..) {
                    k_minus_1_sum.push(nums[i]);

                    #[cfg(test)]
                    k_minus_1_sum.sort();

                    result.push(k_minus_1_sum);
                }

                while i < (nums.len() - k_sum + 1) && nums[i] == nums[i + 1] {
                    i += 1;
                }
                i += 1;
            }
        } else if k_sum == 2 {
            result = Self::two_sum(&nums, target);
        }

        result
    }

    fn two_sum(nums: &[i32], target: i64) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut result = Vec::new();

        while i < j {
            let sum = (nums[i] + nums[j]) as i64;

            if sum == target {
                result.push(vec![nums[i], nums[j]]);
                i += 1;
                while i < j && nums[i] == nums[i - 1] {
                    i += 1;
                }

                j -= 1;
                while j > i && nums[j] == nums[j + 1] {
                    j -= 1;
                }
            } else {
                if sum > target {
                    j -= 1;
                    while j > i && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                } else {
                    i += 1;
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test0() {
        let mut answer = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 100);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test1() {
        let mut answer = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        answer.sort();
        assert_eq!(
            answer,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        let mut answer = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
        answer.sort();
        assert_eq!(answer, vec![vec![2, 2, 2, 2]]);
    }

    #[test]
    fn test3() {
        let mut answer = Solution::four_sum(vec![], 0);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test4() {
        let mut answer = Solution::four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296,
        );
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test5() {
        let mut answer = Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0);
        answer.sort();
        assert_eq!(answer, vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]]);
    }

    #[test]
    fn test6() {
        let mut answer = Solution::four_sum(vec![-5, -2, -4, -2, -5, -4, 0, 0], -13);
        answer.sort();
        assert_eq!(answer, vec![vec![-5, -4, -4, 0], vec![-5, -4, -2, -2]]);
    }
}
