struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut letters = [0; 26];
        let mut diff = 0;

        for c in s.as_bytes() {
            letters[(c - b'a') as usize] += 1;
        }

        for c in t.as_bytes() {
            letters[(c - b'a') as usize] -= 1;
        }

        for count in letters {
            diff += i32::abs(count);
        }

        diff / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::min_steps("bab".to_string(), "aba".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(
            5,
            Solution::min_steps("leetcode".to_string(), "practice".to_string())
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            0,
            Solution::min_steps("anagram".to_string(), "mangaar".to_string())
        );
    }
}
