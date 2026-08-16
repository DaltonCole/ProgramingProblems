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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy_head = ListNode::new(-1);
        let mut even_dummy_head = ListNode::new(-1);
        let mut odd_curr = &mut odd_dummy_head;
        let mut even_curr = &mut even_dummy_head;

        let mut is_even = false;
        while let Some(mut node) = head {
            // head = node.next
            // node.next = None
            //head = std::mem::replace(&mut node.next, None);
            head = node.next.take();
            //head = node.next;
            //node.next = None;
            if is_even {
                even_curr = even_curr.next.insert(node);
                //even_curr.next = Some(node);
                //even_curr = even_curr.next.as_mut().unwrap();
            } else {
                odd_curr = odd_curr.next.insert(node);
                //odd_curr.next = Some(node);
                //odd_curr = odd_curr.next.as_mut().unwrap();
            }
            is_even = !is_even;
        }

        odd_curr.next = even_dummy_head.next;
        odd_dummy_head.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let nodes = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            })),
        }));
        let solution = Solution::odd_even_list(nodes);
        assert_eq!(answer, solution);
    }
}
