struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut longest_start = 0;
        let mut longest_end = 0;
        let mut longest_len = 1;

        for i in 0..chars.len() {
            let (new_start, new_end) = Self::grow_palindrome_odd(&chars, i);
            if (new_end + 1 - new_start) > longest_len {
                longest_start = new_start;
                longest_end = new_end;
                longest_len = new_end - new_start + 1;
            }
        }

        for i in 0..(chars.len() - 1) {
            let (new_start, new_end) = Self::grow_palindrome_even(&chars, i);
            if (new_end + 1 - new_start) > longest_len {
                longest_start = new_start;
                longest_end = new_end;
                longest_len = new_end - new_start + 1;
            }
        }

        return chars[longest_start..=longest_end].iter().collect();
    }

    fn grow_palindrome_odd(chars: &Vec<char>, i: usize) -> (usize, usize) {
        let mut start = i;
        let mut end = i;

        while start > 0 && end < chars.len() - 1 && chars[start - 1] == chars[end + 1] {
            start -= 1;
            end += 1;
        }

        (start, end)
    }

    fn grow_palindrome_even(chars: &Vec<char>, i: usize) -> (usize, usize) {
        let mut start = i;
        let mut end = i + 1;

        if chars[start] != chars[end] {
            return (i, i);
        }

        while start > 0 && end < chars.len() - 1 && chars[start - 1] == chars[end + 1] {
            start -= 1;
            end += 1;
        }

        (start, end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome("cbbe".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }
    #[test]
    fn test4() {
        let s = "eabcb".to_string();
        println!("{s}\n");
        assert_eq!(Solution::longest_palindrome(s), "bcb".to_string());
    }
    #[test]
    fn test5() {
        let s = "aaaaaa".to_string();
        println!("{s}\n");
        assert_eq!(Solution::longest_palindrome(s), "aaaaaa".to_string());
    }
    #[test]
    fn test6() {
        let s = "aaaaaaa".to_string();
        println!("{s}\n");
        assert_eq!(Solution::longest_palindrome(s), "aaaaaaa".to_string());
    }
    #[test]
    fn test7() {
        let s = "aacabdkacaa".to_string();
        println!("{s}\n");
        assert_eq!(Solution::longest_palindrome(s), "aca".to_string());
    }
}
