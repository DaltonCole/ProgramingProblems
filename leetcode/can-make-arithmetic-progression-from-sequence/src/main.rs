struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        if arr.len() <= 2 {
            return true;
        }

        // Get the max and min values
        let max = *arr.iter().max().unwrap();
        let min = *arr.iter().min().unwrap();

        // If all numbers are the same, we're good
        if max == min {
            return true;
        }

        // Get difference between elements
        let diff = (max - min) as f32 / (arr.len() as i32 - 1) as f32;

        // If difference is not an integer, then cannot be done
        if diff.fract() != 0.0 {
            return false;
        }

        let diff = diff as i32;

        // Min + (n * diff) = num
        // n * diff = num - min
        // n = (num - min) / diff
        let mut i = 0;
        while i < arr.len() {
            // Element is in the correct position
            if arr[i] == min + (i as i32 * diff) {
                i += 1;
            }
            // Element does not form a sequence
            else if (arr[i] - min) % diff != 0 {
                return false;
            }
            // Swap elements
            else {
                let correct_index: usize = ((arr[i] - min) / diff).try_into().unwrap();
                // If the correct index is in the past (already filled) return false
                if correct_index < i || arr[i] == arr[correct_index] {
                    return false;
                }
                arr.swap(i, correct_index)
            }
        }

        true
    }

    pub fn n_log_ncan_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();

        let diff = arr[1] - arr[0];

        for i in 2..arr.len() {
            if diff != (arr[i] - arr[i - 1]) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_make_arithmetic_progression(vec![-2, 2, 0]));
    }

    #[test]
    fn test4() {
        assert!(!Solution::can_make_arithmetic_progression(vec![-2, 1, 2]));
    }

    #[test]
    fn test5() {
        assert!(Solution::can_make_arithmetic_progression(vec![0, 0, 0, 0]));
    }

    #[test]
    fn test6() {
        assert!(!Solution::can_make_arithmetic_progression(vec![
            1, 2, 3, 2, 5
        ]));
    }
}
