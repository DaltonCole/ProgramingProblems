#[cfg(test)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();

        match Self::dynamic(&chars[..], &mut HashMap::new()) {
            None => 0,
            Some(x) => x,
        }
    }

    fn dynamic(chars: &[char], map: &mut HashMap<String, Option<i32>>) -> Option<i32> {
        let s = chars.iter().collect::<String>();

        if let Some(&x) = map.get(&s) {
            return x;
        }

        let answer = match chars.len() {
            0 => Some(1),
            1 => match chars[0] {
                '0' => None,
                _ => Some(1),
            },
            _ => {
                // Two zeros => None
                // Leading zero => None
                if chars[0] == '0' {
                    None
                }
                // [3-9]X => Consume 1
                // 2[7-9] => Consume 1
                else if chars[0] >= '3' || (chars[0] == '2' && chars[1] >= '7') {
                    match Self::dynamic(&chars[1..], map) {
                        None => None,
                        Some(x) => Some(x),
                    }
                }
                // [1, 2]0 => Consume 2
                else if (chars[0] == '1' || chars[0] == '2') && chars[1] == '0' {
                    match Self::dynamic(&chars[2..], map) {
                        None => None,
                        Some(x) => Some(x),
                    }
                }
                // Two non-zeros, try consuming 1 and two characters
                else {
                    let consume_single = Self::dynamic(&chars[1..], map);
                    let consume_both = Self::dynamic(&chars[2..], map);

                    match (consume_single, consume_both) {
                        (None, None) => None,
                        (Some(x), None) | (None, Some(x)) => Some(x),
                        (Some(x), Some(y)) => Some(x + y),
                    }
                }
            }
        };

        map.insert(s, answer);

        #[cfg(test)]
        println!("{answer:?}: {chars:?}");

        return answer;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::num_decodings("11106".to_string()), 2);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::num_decodings("11006".to_string()), 0);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::num_decodings("99999999999".to_string()), 1);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::num_decodings("123123".to_string()), 9);
    }
}
