struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut size = nums.len();
        let mut mem = vec![0; size];

        for index in 0..size {
            size -= 1;
            match index {
                0 => mem[index] = nums[size],
                1 => {
                    mem[index] = std::cmp::max(nums[size], nums[size + 1]);
                }
                _ => mem[index] = std::cmp::max(nums[size] + mem[index - 2], mem[index - 1]),
            }
        }
        *mem.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }

    #[test]
    fn test3() {
        assert_eq!(30, Solution::rob(vec![10, 1, 1, 10, 1, 10, 1]));
    }
}
