// Definition for singly-linked list.
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

impl Solution {
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut size = 0;
        while let Some(x) = head {
            size += 1;
            head = &x.next;
        }
        size
    }

    fn half_way(head: &Option<Box<ListNode>>, length: usize) -> &Option<Box<ListNode>> {
        let mut ptr = head;
        for _ in 0..(length / 2) {
            if let Some(node) = ptr {
                ptr = &node.next;
            }
        }
        ptr
    }

    fn reverse(head: &Option<&mut Box<ListNode>>) {
        let mut prev = head;
        let mut current = head.unwrap().next.as_mut();
        while let Some(x) = current {
            let next = x.next;
        }
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Get size of linked list
        let size = Self::length(head);

        // --- Reverse second half of points --- //
        // Go to half way point
        let mut half = Self::half_way(head, size).as_mut();
        // Reverse second half
        Self::reverse(&half);

        /*
                let mut prev = current;
                current = &mut prev.unwrap().as_ref().next;
                let mut next = &mut prev.unwrap().as_ref().next;
                while let Some(x) = next {
                    current.unwrap().next = *prev;
                    prev = current;
                    current = next;
                    *next = current.unwrap().next;
                }
        */

        // --- Move second half of points to the intertwine with the first half --- //
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn helper(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;
        for num in nums {
            current.next = Some(Box::new(ListNode::new(num)));
            current = current.next.as_mut().unwrap();
        }
        front.next
    }

    #[test]
    fn test1() {
        let mut head = helper(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, helper(vec![1, 4, 2, 3]));
    }

    #[test]
    fn test2() {
        let mut head = helper(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut head);
        assert_eq!(head, helper(vec![1, 5, 2, 4, 3]));
    }
}
