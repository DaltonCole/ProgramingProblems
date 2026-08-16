struct Solution;

use std::collections::HashSet;

// Pre-fix tree is another possible solution

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut words = HashSet::new();

        for word in word_dict {
            words.insert(word);
        }

        let mut good_subset = vec![false; s.len() + 1];
        good_subset[0] = true;

        for i in 1..(s.len() + 1) {
            for j in (0..i).rev() {
                if good_subset[j] {
                    if words.contains(&s[j..i]) {
                        good_subset[i] = true;
                        break;
                    }
                }
            }
        }

        good_subset[s.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn str(str_vec: Vec<&str>) -> Vec<String> {
        str_vec.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            str(vec!["leet", "code"])
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            str(vec!["apple", "pen"])
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            str(vec!["cats", "dog", "sand", "and", "cat"])
        ));
    }

    #[test]
    fn test4() {
        assert!(!Solution::word_break(
            "e".to_string(),
            str(vec!["leet", "code"])
        ));
    }

    #[test]
    fn test5() {
        assert!(Solution::word_break("e".to_string(), str(vec!["e"])));
    }

    #[test]
    fn test6() {
        assert!(Solution::word_break(
            "aaaaaaa".to_string(),
            str(vec!["aaaa", "aaa"])
        ));
    }

    #[test]
    fn test7() {
        assert!(!Solution::word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
            str(vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"])
        ));
    }
}
