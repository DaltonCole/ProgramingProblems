struct Solution;

// -4294967296 => Impossible to get since the 6 in front wouldn't fit in i32
// 4294967295
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Self::int_reverse(x)
        //Self::str_reverse(x)
    }

    fn int_reverse(mut x: i32) -> i32 {
        let mut y: i32 = 0;

        while x != 0 {
            match y.checked_mul(10) {
                Some(num) => match num.checked_add(x % 10) {
                    Some(num2) => {
                        y = num2;
                    }
                    None => {
                        return 0;
                    }
                },
                None => {
                    return 0;
                }
            }
            x /= 10;
        }

        y
    }

    fn str_reverse(mut x: i32) -> i32 {
        let mut char_x = Vec::<char>::new();
        let mut final_num: i32 = 0;

        // Remember if negative
        let neg = x < 0;
        x = x.abs();

        // Convert x to a character list
        while x > 0 {
            char_x.insert(0, char::from_digit((x % 10) as u32, 10).unwrap());
            x /= 10;
        }

        // Remove trailing 0s
        while let Some(&x) = char_x.last() {
            if x == '0' {
                char_x.pop();
            } else {
                break;
            }
        }

        // Reverse it
        char_x.reverse();

        // Calculate final value
        for c in char_x {
            match final_num.checked_mul(10) {
                Some(num) => {
                    final_num = num;
                }
                None => {
                    return 0;
                }
            }

            match final_num.checked_add(c.to_digit(10).unwrap() as i32) {
                Some(num) => {
                    final_num = num;
                }
                None => {
                    return 0;
                }
            }
        }

        // Add - sign in front and check
        if neg {
            final_num *= -1
        }

        final_num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(321, Solution::reverse(123));
    }
    #[test]
    fn test2() {
        assert_eq!(-321, Solution::reverse(-123));
    }
    #[test]
    fn test3() {
        assert_eq!(21, Solution::reverse(120));
    }
    #[test]
    fn test4() {
        assert_eq!(0, Solution::reverse(1234567899));
    }
    #[test]
    fn test5() {
        assert_eq!(0, Solution::reverse(-1234567899));
    }
}
