use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();

        for c in s.chars() {
            let value = map.entry(c).or_insert(0);
            *value += 1;
        }

        for c in t.chars() {
            if !map.contains_key(&c) {
                return false;
            }

            let value = map.entry(c).or_insert(-1);
            *value -= 1;
            if *value < 0 {
                return false;
            }
        }

        map.values().find(|&&x| x != 0).is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }
    #[test]
    fn test2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}

// Compact solution: https://leetcode.com/problems/valid-anagram/discuss/360434/rust
// Super compact solution: https://leetcode.com/problems/valid-anagram/discuss/1975028/Rust-solution
