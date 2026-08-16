struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut answer = 0;

        for val in left {
            answer = std::cmp::max(answer, val);
        }

        for val in right {
            answer = std::cmp::max(answer, n - val);
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, Solution::get_last_moment(4, vec![4, 3], vec![0, 1]));
    }
    #[test]
    fn test2() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7])
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![])
        );
    }
    #[test]
    fn test4() {
        assert_eq!(0, Solution::get_last_moment(1000, vec![0], vec![]));
    }
}
