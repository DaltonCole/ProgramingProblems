struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut occurrences = vec![0; 26];

        for c in s.chars() {
            occurrences[c as usize - 97] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if occurrences[c as usize - 97] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!(-1, Solution::first_uniq_char("aabb".to_string()));
    }
}
