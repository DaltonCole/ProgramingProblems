struct Solution;

impl Solution {
    // Since this is rust, should return an option in case len()
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        match Self::safe_three_sum_closest(&mut nums, target) {
            None => target,
            Some(x) => x,
        }
    }

    fn safe_three_sum_closest(nums: &mut Vec<i32>, target: i32) -> Option<i32> {
        let mut closest = None;
        let mut closest_diff = 0;

        nums.sort_unstable();

        let mut current = 0;
        while current < nums.len() - 2 {
            let mut left = current + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[current] + nums[left] + nums[right];
                let diff = (sum - target).abs();

                #[cfg(test)]
                println!(
                    "{} + {} + {} = {}",
                    nums[current], nums[left], nums[right], sum
                );

                match closest {
                    None => {
                        closest = Some(sum);
                        closest_diff = diff;
                    }
                    Some(_x) => {
                        if closest_diff > diff {
                            closest = Some(sum);
                            closest_diff = diff;
                        }
                    }
                }

                if sum == target {
                    return closest;
                }

                if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }

            // Skip duplicate numbers
            while current < nums.len() - 2 && nums[current] == nums[current + 1] {
                current += 1;
            }
            current += 1;
        }

        closest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let answer = Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test2() {
        let answer = Solution::three_sum_closest(vec![0, 0, 0], 2);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test3() {
        let answer = Solution::three_sum_closest(vec![-1, 2, 1, -4, -9, -9, -9, -9], 1);
        assert_eq!(answer, 2);
    }
}
