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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;

        let mut l1 = &list1;
        let mut l2 = &list2;

        loop {
            match (l1, l2) {
                (Some(x), Some(y)) => match x.val < y.val {
                    true => {
                        current.next = Some(Box::new(ListNode::new(x.val)));
                        current = current.next.as_mut().unwrap();
                        l1 = &x.next;
                    }
                    false => {
                        current.next = Some(Box::new(ListNode::new(y.val)));
                        current = current.next.as_mut().unwrap();
                        l2 = &y.next;
                    }
                },
                (Some(x), None) => {
                    current.next = Some(Box::new(ListNode::new(x.val)));
                    current = current.next.as_mut().unwrap();
                    l1 = &x.next;
                }
                (None, Some(x)) => {
                    current.next = Some(Box::new(ListNode::new(x.val)));
                    current = current.next.as_mut().unwrap();
                    l2 = &x.next;
                }
                (None, None) => {
                    break;
                }
            }
        }

        front.next
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
        assert_eq!(
            Solution::merge_two_lists(helper(vec![1, 2, 4]), helper(vec![1, 3, 4])),
            helper(vec![1, 1, 2, 3, 4, 4])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge_two_lists(helper(vec![]), helper(vec![])),
            helper(vec![])
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::merge_two_lists(helper(vec![]), helper(vec![0])),
            helper(vec![0])
        );
    }
}
