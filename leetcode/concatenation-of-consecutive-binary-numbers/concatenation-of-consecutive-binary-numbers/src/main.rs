struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut total = 1;
        let MOD = 1000000007;

        for i in 2..n + 1 {
            let mut tmp = i;
            while tmp > 0 {
                total <<= 1;
                total %= MOD;
                tmp >>= 1;
            }
            total = (total + i) % MOD;
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::concatenated_binary(1), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::concatenated_binary(3), 27);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }
}
