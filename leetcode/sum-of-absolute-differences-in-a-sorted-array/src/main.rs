struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        Self::best(&nums)
        //Self::better_only_10000_nums(&nums)
        //Self::only_10000_nums(&nums)
        //Self::naieve_solution(&nums)
    }

    pub fn best(nums: &Vec<i32>) -> Vec<i32> {
        let mut solution = vec![0; nums.len()];

        solution[0] = nums.iter().map(|x| x - nums[0]).sum();

        for i in 1..nums.len() {
            solution[i] = solution[i - 1] + ((nums[i] - nums[i - 1]) * i as i32)
                - ((nums[i] - nums[i - 1]) * (nums.len() - i) as i32);
        }

        solution
    }

    pub fn better_only_10000_nums(nums: &Vec<i32>) -> Vec<i32> {
        let mut solution = vec![0; nums.len()];

        // Odd
        if nums.len() % 2 == 1 {
            let middle_index = nums.len() / 2;

            solution[middle_index] = nums.iter().map(|x| (x - nums[middle_index]).abs()).sum();

            for i in (0..middle_index).rev() {
                solution[i] = solution[i + 1]
                    + ((nums[i + 1] - nums[i]) * (1 + (2 * (middle_index - i - 1) as i32)));
            }

            for i in middle_index + 1..nums.len() {
                solution[i] = solution[i - 1]
                    + ((nums[i] - nums[i - 1]) * (1 + (2 * (i - middle_index - 1) as i32)));
            }
        } else {
            let mut middle_index = nums.len() / 2;

            solution[middle_index] = nums.iter().map(|x| (x - nums[middle_index]).abs()).sum();
            solution[middle_index - 1] = solution[middle_index];

            middle_index -= 1;
            for i in (0..middle_index).rev() {
                solution[i] =
                    solution[i + 1] + ((nums[i + 1] - nums[i]) * (2 * (middle_index - i) as i32));
            }

            middle_index += 1;
            for i in middle_index + 1..nums.len() {
                solution[i] =
                    solution[i - 1] + ((nums[i] - nums[i - 1]) * (2 * (i - middle_index) as i32));
            }
        }

        solution
    }

    pub fn only_10000_nums(nums: &Vec<i32>) -> Vec<i32> {
        let mut solution = vec![0; nums.len()];

        let mut counts = vec![0; 10001];

        for &num in nums.iter() {
            counts[num as usize] += 1;
        }

        for (i, &num) in nums.iter().enumerate() {
            // If the previous num is the same as this num, just use that solution
            if i > 1 && num == nums[i - 1] {
                solution[i] = solution[i - 1];
                continue;
            }

            let mut diff = 0;
            for (j, &count) in counts.iter().enumerate() {
                diff += (num - j as i32).abs() * count;
            }
            solution[i] = diff;
        }

        solution
    }

    pub fn naieve_solution(nums: &Vec<i32>) -> Vec<i32> {
        let mut solution = vec![0; nums.len()];

        for (i, &num1) in nums.iter().enumerate() {
            let mut diff = 0;
            for &num2 in nums.iter() {
                diff += (num1 - num2).abs()
            }
            solution[i] = diff;
        }

        solution
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5, 10]),
            vec![12, 10, 10, 20]
        );
    }

    #[test]
    fn test01() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5, 6, 8, 12]),
            vec![24, 20, 16, 16, 20, 36]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5, 7, 8, 12]),
            vec![25, 21, 17, 17, 19, 35]
        );
    }

    #[test]
    fn test03() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 2, 2, 3, 3, 5, 7, 8, 8, 12]),
            vec![32, 32, 32, 28, 28, 28, 32, 36, 36, 68]
        );
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![2, 3, 5]),
            vec![4, 3, 5]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
            vec![24, 15, 13, 15, 21]
        );
    }
}
