struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut map = HashMap::new();

        for num in &nums {
            map.entry(num % space).and_modify(|x| *x += 1).or_insert(1);
        }

        let value = *map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(_k, v)| v)
            .unwrap();

        let mut keys = HashSet::new();
        for (&x, &y) in &map {
            if y == value {
                keys.insert(x);
            }
        }

        let mut answer = std::i32::MAX;

        for num in &nums {
            if keys.contains(&(num % space)) {
                answer = answer.min(*num);
            }
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::destroy_targets(vec![6, 2, 5], 100), 2);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::destroy_targets(vec![1, 5, 3, 2, 2], 10000), 2);
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::destroy_targets(vec![10, 11, 20, 22, 30, 33, 44], 2),
            10
        );
    }
    #[test]
    fn test6() {
        assert_eq!(
            Solution::destroy_targets(vec![11, 20, 22, 30, 33, 44], 2),
            20
        );
    }
    #[test]
    fn test7() {
        assert_eq!(Solution::destroy_targets(vec![5, 2], 4), 2);
    }
}
