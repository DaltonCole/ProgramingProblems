struct Solution;

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n % 2 == 1 {
                count += 1;
            }
            println!("{}", n);
            n = n >> 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, Solution::hammingWeight(0));
        assert_eq!(1, Solution::hammingWeight(1));
        assert_eq!(1, Solution::hammingWeight(2));
        assert_eq!(2, Solution::hammingWeight(3));
        assert_eq!(1, Solution::hammingWeight(4));
        assert_eq!(3, Solution::hammingWeight(11));
    }
}
