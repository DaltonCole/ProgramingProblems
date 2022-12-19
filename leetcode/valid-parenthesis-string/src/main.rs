struct Solution;

// I CHEATED AND LOOKED AT THE SOLUTION :'(

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut lo = 0;
        let mut hi = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    lo += 1;
                    hi += 1;
                }
                ')' => {
                    lo -= 1;
                    hi -= 1;
                }
                '*' => {
                    lo -= 1;
                    hi += 1;
                }
                _ => {}
            }

            lo = std::cmp::max(lo, 0);

            if hi < 0 {
                return false;
            }
        }

        lo == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_valid_string("()".to_string()));
    }
    #[test]
    fn test2() {
        assert!(Solution::check_valid_string("(*)".to_string()));
    }
    #[test]
    fn test3() {
        assert!(Solution::check_valid_string("(*))".to_string()));
    }
    #[test]
    fn test4() {
        assert!(!Solution::check_valid_string("())".to_string()));
    }
    #[test]
    fn test5() {
        assert!(!Solution::check_valid_string("(".to_string()));
    }
    #[test]
    fn test6() {
        assert!(Solution::check_valid_string("*".to_string()));
    }
    #[test]
    fn test7() {
        assert!(!Solution::check_valid_string("))".to_string()));
    }
    #[test]
    fn test8() {
        assert!(Solution::check_valid_string("((*)".to_string()));
    }
    #[test]
    fn test9() {
        assert!(Solution::check_valid_string("((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()".to_string()));
    }
    #[test]
    fn test10() {
        assert!(!Solution::check_valid_string("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_string()));
    }
    #[test]
    fn test11() {
        assert!(!Solution::check_valid_string(")*".to_string()));
    }
    #[test]
    fn test12() {
        assert!(Solution::check_valid_string("(()(())()())*((()(())))*()(*)()()(*((()((*(*))))()*()(()((()(*((()))*(((())(())))*))(()*))(()*)".to_string()));
    }
    #[test]
    fn test13() {
        assert!(Solution::check_valid_string(
            "(()(())()())*((()(())))*()".to_string()
        ));
    }
    #[test]
    fn test14() {
        assert!(Solution::check_valid_string(
            "**(*)(**(**((***(*))(*)".to_string()
        ));
    }
}
