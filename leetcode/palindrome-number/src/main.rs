struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut rev = 0;
        let mut ori = x;

        while ori > 0 {
            rev *= 10;
            let num = ori % 10;
            ori /= 10;
            rev += num;
        }

        rev == x
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(121));
    }
    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome(-121));
    }
    #[test]
    fn test3() {
        assert!(!Solution::is_palindrome(10));
    }
    #[test]
    fn test4() {
        assert!(Solution::is_palindrome(1));
    }
    #[test]
    fn test5() {
        assert!(Solution::is_palindrome(11));
    }
}
