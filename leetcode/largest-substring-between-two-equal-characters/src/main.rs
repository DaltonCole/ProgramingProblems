struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max_dist: i32 = -1;

        let mut first_occurrence = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            match first_occurrence.get(&c) {
                Some(j) => {
                    max_dist = std::cmp::max(max_dist, i as i32 - j - 1);
                }
                None => {
                    first_occurrence.insert(c, i as i32);
                }
            }
        }

        max_dist
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            0,
            Solution::max_length_between_equal_characters("aa".to_string())
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            2,
            Solution::max_length_between_equal_characters("abca".to_string())
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            -1,
            Solution::max_length_between_equal_characters("cbzxy".to_string())
        );
    }
}
