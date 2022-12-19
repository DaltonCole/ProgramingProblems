#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.val).cmp(&Reverse(other.val))
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Reverse(self.val).cmp(&Reverse(other.val)))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        // Add first item in each list to a priority queue
        let mut pq = BinaryHeap::new();
        for list in lists {
            if let Some(x) = list {
                pq.push(x);
            }
        }

        // Empty front
        let mut front = Box::new(ListNode::new(0));
        // Pointer to current item in return list
        let mut current = &mut front;

        // While there are items to process
        while let Some(top) = pq.pop() {
            // Set the current pointer's next to this
            current.next = Some(Box::new(ListNode::new(top.val)));
            // Update current to next
            current = current.next.as_mut().unwrap();
            // Push next item in the grabbed list, if there is one
            if let Some(next) = top.next {
                pq.push(next);
            }
        }

        front.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn helper(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result = None;
        for num in nums.iter().rev() {
            result = Some(Box::new(ListNode {
                val: *num,
                next: result,
            }));
        }
        result
    }

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                helper(vec![1, 4, 5]),
                helper(vec![1, 3, 4]),
                helper(vec![2, 6])
            ]),
            helper(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::merge_k_lists(vec![]), helper(vec![]));
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::merge_k_lists(vec![helper(vec![])]),
            helper(vec![])
        );
    }
}
