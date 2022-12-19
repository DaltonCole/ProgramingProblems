struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let mut index;
        let num_rows = num_rows as usize;

        for row in (0..num_rows).rev() {
            index = num_rows - row - 1;
            let mut down = false;

            let first_jump = 2 * row;
            let second_jump = (2 * (num_rows - 1)) - first_jump;

            while index < chars.len() {
                result.push(chars[index]);

                if row == (num_rows - 1) || row == 0 {
                    index += 2 * (num_rows - 1);
                } else {
                    if !down {
                        index += first_jump;
                    } else {
                        index += second_jump;
                    }
                }

                down = !down;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::convert("".to_string(), 3), "".to_string());
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}
