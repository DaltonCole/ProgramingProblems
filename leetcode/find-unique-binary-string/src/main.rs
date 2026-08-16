struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut solution = String::new();
        let mut contains = vec![false; 2_i32.pow(16) as usize];
        let len = nums[0].len();

        for num in nums {
            let num = u16::from_str_radix(&num, 2).unwrap();
            contains[num as usize] = true;
        }

        let mut ans = contains.iter().position(|&x| x == false).unwrap();

        while ans > 0 {
            if ans % 2 == 1 {
                solution.push('1');
            } else {
                solution.push('0');
            }
            ans /= 2;
        }

        solution = solution.chars().rev().collect();

        while solution.len() < len {
            solution = "0".to_string() + &solution;
        }

        solution
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            "00",
            Solution::find_different_binary_string(vec!["01".to_string(), "10".to_string()])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            "10",
            Solution::find_different_binary_string(vec!["01".to_string(), "00".to_string()])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            "000",
            Solution::find_different_binary_string(vec![
                "111".to_string(),
                "011".to_string(),
                "001".to_string()
            ])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            "010",
            Solution::find_different_binary_string(vec![
                "000".to_string(),
                "001".to_string(),
                "110".to_string()
            ])
        );
    }
}
