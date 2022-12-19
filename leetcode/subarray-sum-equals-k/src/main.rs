struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        #[cfg(test)]
        println!("{nums:?}, {k}");
        //return Self::slow(&nums, k);
        //return Self::faster(&nums, k);
        return Self::even_faster(&nums, k);
    }

    fn even_faster(nums: &Vec<i32>, k: i32) -> i32 {
        let mut count: i32 = 0;

        // Keep track of number of occurrences of each sum
        let mut sums: HashMap<i32, i32> = HashMap::new();
        // Starting sum is 0, so add it to the map
        sums.insert(0, 1);

        let mut sum = 0;
        for num in nums {
            sum += num;

            // Find X where x + sum = k
            let opposite = sum - k;
            // If X exists, then there are N ways to sum up to k
            if let Some(sum_count) = sums.get(&opposite) {
                count += sum_count;
            }

            *(sums.entry(sum).or_insert(0)) += 1;
        }

        count
    }

    fn faster(nums: &Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        // Keep track of the indexes of all sums
        let mut sums: HashMap<i32, Vec<i32>> = HashMap::new();
        sums.insert(0, vec![-1]); // Start off at sum = 0

        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            sums.entry(sum).or_insert(Vec::new()).push(i as i32);
        }

        #[cfg(test)]
        println!("{:?}", sums);

        for (sum, indexes) in &sums {
            let opposite = sum - k;
            // Get all the indexes of X where X + sum = k
            if let Some(opposite_indexes) = sums.get(&opposite) {
                for index1 in indexes {
                    for index2 in opposite_indexes {
                        // Maintain order so we don't count twice
                        if index1 > index2 {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn slow(nums: &Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for (i, &num1) in nums.iter().enumerate() {
            let mut sum = num1;
            if sum == k {
                count += 1;
            }
            for &num2 in nums.iter().skip(i + 1) {
                sum += num2;
                if sum == k {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::subarray_sum(vec![], 2), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 100), 0);
    }
    #[test]
    fn test5() {
        assert_eq!(Solution::subarray_sum(vec![99, 1, 99], 99), 2);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1, 1, 1, 1], 2), 5);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::subarray_sum(vec![1, -1, -1, 1], 0), 3);
    }
    #[test]
    fn test8() {
        assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0], 0), 10);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::subarray_sum(vec![0, 1, 0, 0], 0), 4);
    }
    #[test]
    fn test10() {
        assert_eq!(Solution::subarray_sum(vec![7], 7), 1);
    }
}
