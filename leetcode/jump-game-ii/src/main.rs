struct Solution;

// Greedy
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut next_step_index = 0;
        let mut jumping_distance = 0;

        for i in 0..(nums.len() - 1) {
            jumping_distance = jumping_distance.max(i + nums[i] as usize);

            if i >= next_step_index {
                steps += 1;
                next_step_index = jumping_distance;
            }
        }
        #[cfg(test)]
        println!("{:?}", nums);
        #[cfg(test)]
        println!("{}", steps);

        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }
    #[test]
    fn test2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
    #[test]
    fn test3() {
        assert_eq!(1, Solution::jump(vec![100, 0, 0, 0, 0]));
    }
    #[test]
    fn test4() {
        assert_eq!(0, Solution::jump(vec![0]));
    }
}
