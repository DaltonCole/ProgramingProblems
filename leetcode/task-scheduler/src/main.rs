struct Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }

        // Count the occurrences of each character
        let mut map = HashMap::new();
        for c in &tasks {
            *map.entry(c).or_insert(0) += 1;
        }

        // Add characters and values added to a heap
        let mut heap = BinaryHeap::new();

        for (c, count) in &map {
            heap.push((count, c));
        }

        let mut v = Vec::<bool>::new();
        v.reserve(2000000);

        while let Some((c, count)) = heap.pop() {
            let mut index = 0;
            for 
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::least_interval(vec!['a', 'a', 'a', 'b', 'b', 'b'], 2),
            8
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::least_interval(vec!['a', 'a', 'a', 'b', 'b', 'b'], 0),
            6
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::least_interval(
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'b', 'c', 'd', 'e', 'f', 'g'],
                2
            ),
            16
        );
    }
}
