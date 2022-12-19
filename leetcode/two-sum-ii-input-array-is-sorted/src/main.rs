struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = nums.len() - 1;

        while i < j {
            let sum = nums[i] + nums[j];

            #[cfg(test)]
            println!("{} = {} + {}", sum, nums[i], nums[j]);

            if sum == target {
                return vec![i as i32 + 1, j as i32 + 1];
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
        assert_eq!(answer, vec![1, 2]);
    }

    #[test]
    fn test1() {
        let mut answer = Solution::two_sum(vec![2, 3, 4], 6);
        answer.sort();
        assert_eq!(answer, vec![1, 3]);
    }

    #[test]
    fn test2() {
        let mut answer = Solution::two_sum(vec![-1, 0], -1);
        answer.sort();
        assert_eq!(answer, vec![1, 2]);
    }
}
