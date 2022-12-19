struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut running_total = max;

        for &num in nums.iter().skip(1) {
            running_total = std::cmp::max(running_total + num, num);
            max = std::cmp::max(max, running_total);
            #[cfg(test)]
            println!("{}: {} {}", num, running_total, max);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }

    #[test]
    fn test3() {
        assert_eq!(23, Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
    }

    #[test]
    fn test4() {
        assert_eq!(
            4,
            Solution::max_sub_array(vec![-2, 2, -3, 4, -9, 2, 1, -5, 4])
        );
    }
}
