struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.reserve(k as usize);
        for num in nums {
            if heap.len() < k as usize {
                heap.push(Reverse(num));
            } else {
                if heap.peek().unwrap().0 < num {
                    heap.pop();
                    heap.push(Reverse(num));
                }
            }
        }
        heap.peek().unwrap().0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
    }
}
