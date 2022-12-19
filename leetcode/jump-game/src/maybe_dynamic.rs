struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jumpable = vec![false; nums.len()];
        *(jumpable.last_mut().unwrap()) = true;

        for index in (0..jumpable.len() - 1).rev() {
            jumpable[index] = jumpable
                [index..(index + nums[index] as usize + 1).min(jumpable.len())]
                .iter()
                .any(|&x| x == true);
        }

        #[cfg(test)]
        println!("{:?}", nums);
        #[cfg(test)]
        println!("{:?}", jumpable);

        jumpable[0]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn test4() {
        assert!(Solution::can_jump(vec![3, 2, 1, 0]));
    }

    #[test]
    fn test5() {
        assert!(Solution::can_jump(vec![4, 2, 0, 0, 1, 1, 4, 4, 4, 0, 4, 0]));
    }
}
