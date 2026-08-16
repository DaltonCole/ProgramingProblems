struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut v = vec![2, 3, 3, 2];
        let len = Solution::remove_element(&mut v, 3);
        assert_eq!(len, 2);
        assert_eq!(v, vec![2, 2]);
    }
}
