struct Solution;

// Dynamic Programming
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = vec![i16::MAX as i32; nums.len()];
        *(jumps.last_mut().unwrap()) = 0;

        for i in (0..jumps.len() - 1).rev() {
            println!("{}: {}", i, nums[i]);
            if nums[i] != 0 {
                jumps[i] = jumps[(i + 1)..(i + nums[i] as usize + 1).min(jumps.len())]
                    .iter()
                    .min()
                    .unwrap()
                    + 1;
            }
        }

        #[cfg(test)]
        println!("{:?}", nums);
        #[cfg(test)]
        println!("{:?}", jumps);

        jumps[0]
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
}
