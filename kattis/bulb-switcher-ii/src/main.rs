struct Solution;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::flip_lights(1, 1));
    }
    #[test]
    fn test2() {
        assert_eq!(3, Solution::flip_lights(2, 1));
    }
    #[test]
    fn test3() {
        assert_eq!(4, Solution::flip_lights(3, 1));
    }
    #[test]
    fn test4() {
        assert_eq!(8, Solution::flip_lights(5, 5));
    }
    #[test]
    fn test5() {
        assert_eq!(8, Solution::flip_lights(6, 6));
    }
    #[test]
    fn test6() {
        assert_eq!(8, Solution::flip_lights(7, 7));
    }
    #[test]
    fn test7() {
        assert_eq!(4, Solution::flip_lights(5, 1));
    }
    #[test]
    fn test8() {
        assert_eq!(7, Solution::flip_lights(5, 2));
    }
    #[test]
    fn test9() {
        assert_eq!(8, Solution::flip_lights(5, 3));
    }
}
