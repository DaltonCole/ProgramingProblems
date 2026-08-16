struct Solution;

enum Median {
    First,
    Second,
    Both,
    FirstLess,
    FirstGreater,
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let mut index1 = (nums1.len() / 2) as i32;
        let mut index2 = (nums2.len() / 2) as i32;

        loop {
            let num1 = nums1.get(index1);
            let num2 = nums2.get(index2);
            let num1_less = nums1.get(index1 - 1);
            let num1_greater = nums1.get(index1 + 1);
            let num2_less = nums2.get(index1 - 1);
            let num2_greater: Option<&i32> = nums2.get(index1 + 1);
            match Solution::is_median(num1, num2, num1_less, num1_greater, num2_less, num2_greater)
            {
                Median::First => return nums1[index1] as f64,
                Median::Second => return nums2[index2] as f64,
                Median::Both => return (nums1[index1] as f64 + nums2[index2] as f64) / 2.0,
                Median::FirstLess => {
                    continue;
                }
                Median::FirstGreater => continue,
            }
        }
    }

    fn is_median(
        num1: Option<&i32>,
        num2: Option<&i32>,
        num1_less: Option<&i32>,
        num1_greater: Option<&i32>,
        num2_less: Option<&i32>,
        num2_greater: Option<&i32>,
    ) -> Median {
        // No first number
        if let None = num1 {
            return Median::Second;
        }
        // No Second number
        if let None = num2 {
            return Median::First;
        }
        Median::First
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2,]),
            2.0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![10, 11, 12, 13, 14, 15]
            ),
            8.0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![1, 3, 5, 7, 8, 9, 11, 13, 15],
                vec![2, 4, 6, 10, 12, 14]
            ),
            8.0
        );
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,], vec![2,]), 1.5);
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![10, 11, 12, 13, 14, 15],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ),
            8.0
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![2, 4, 6, 10, 12, 14],
                vec![1, 3, 5, 7, 8, 9, 11, 13, 15],
            ),
            8.0
        );
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1],), 1.0);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![],), 1.0);
    }
}
