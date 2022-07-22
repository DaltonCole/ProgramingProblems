use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    nums: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
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
        KthLargest { k, nums: heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        // Need more elements, so add val to heap
        if self.nums.len() < self.k as usize {
            self.nums.push(Reverse(val));
        }
        // New value is larger than the heap's smallest element
        else if val > self.nums.peek().unwrap().0 {
            // Remove smallest number
            self.nums.pop();
            // Add larger val to heap
            self.nums.push(Reverse(val));
        }
        // Return the smallest of the k elements
        return self.nums.peek().unwrap().0;
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut x = KthLargest::new(3, [4, 5, 8, 3].to_vec());
        assert_eq!(4, x.add(3));
        assert_eq!(5, x.add(5));
        assert_eq!(5, x.add(10));
        assert_eq!(8, x.add(9));
        assert_eq!(8, x.add(4));
    }

    #[test]
    fn test2() {
        let mut x = KthLargest::new(3, [5, 8].to_vec());
        assert_eq!(4, x.add(4));
        assert_eq!(5, x.add(5));
        assert_eq!(5, x.add(10));
        assert_eq!(8, x.add(9));
        assert_eq!(8, x.add(4));
    }
}
