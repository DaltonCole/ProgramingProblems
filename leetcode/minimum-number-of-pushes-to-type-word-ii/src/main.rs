struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0; 26];
        let mut total = 0;

        for c in word.as_bytes() {
            counts[(c - b'a') as usize] += 1
        }

        counts.sort_unstable();

        for (buttons, count) in counts.iter().rev().enumerate() {
            let modifier = (buttons / 8) + 1;
            total += modifier * count;
        }

        total as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            24,
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string())
        );
    }
    #[test]
    fn test2() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
    }
}
