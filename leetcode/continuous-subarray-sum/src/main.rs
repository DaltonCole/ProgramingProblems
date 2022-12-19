struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        //return Self::slow_Onn(&nums, k);
        //return Self::slow_On_min_k_n(&nums, k);
        return Self::fast(&nums, k);
    }

    fn fast(nums: &Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        let mut sum = 0;

        map.insert(0, 0);

        for (i, num) in nums.iter().enumerate() {
            sum += num;
            sum %= k;

            #[cfg(test)]
            println!("{sum}");

            match map.get(&sum) {
                // If we haven't seen the accumulated modded sum yet, add it
                None => {
                    map.insert(sum, i + 1);
                }
                // If we have already seen the accumulated modded sum, then the range is a
                // multiple of k
                Some(&x) => {
                    // Make sure the range is longer than 1
                    if x < i {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn slow_On_min_k_n(nums: &Vec<i32>, k: i32) -> bool {
        if nums.len() <= 1 {
            return false;
        }

        let mut running_sum = HashSet::new();

        for num in nums {
            let mut current_sum = HashSet::new();

            for old_sum in &running_sum {
                let sum = (num + old_sum) % k;

                if sum == 0 {
                    return true;
                }

                current_sum.insert(sum);
            }

            current_sum.insert(num % k);
            running_sum = current_sum;

            #[cfg(test)]
            println!("{running_sum:?}");
        }

        false
    }

    fn slow_Onn(nums: &Vec<i32>, k: i32) -> bool {
        if nums.len() <= 1 {
            return false;
        }

        for i in 0..(nums.len() - 1) {
            let mut total = nums[i];
            for j in (i + 1)..(nums.len()) {
                total += nums[j];

                if total % k == 0 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn test2() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }
    #[test]
    fn test3() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }
    #[test]
    fn test4() {
        assert!(Solution::check_subarray_sum(vec![1, 2, 3, 4, 0, 0], 100));
    }
    #[test]
    fn test5() {
        assert!(!Solution::check_subarray_sum(vec![0], 100));
    }
    #[test]
    fn test6() {
        assert!(!Solution::check_subarray_sum(vec![], 10));
    }
    #[test]
    fn test7() {
        assert!(Solution::check_subarray_sum(vec![0, 0], 100));
    }
    #[test]
    fn test8() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 6], 7));
    }
    #[test]
    fn test9() {
        assert!(!Solution::check_subarray_sum(vec![1, 0], 2));
    }
}
