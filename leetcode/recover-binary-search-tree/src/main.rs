use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut max = None;
        let bad;
        let parent = root.clone();
        let mut queue = Vec::new();
        let mut went_left = false;

        queue.push(parent);

        while !queue.is_empty() {
            let node = queue.pop().unwrap();

            if !went_left {
                if let Some(x) = node.borrow().left {}
            }
        }

        loop {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn array_to_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap()))));
        let root2 = root.clone();

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        let mut i = 1;
        while i < arr.len() {
            let curr = queue.pop_front().unwrap();

            if i < arr.len() {
                if let Some(x) = arr[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(x)));
                    curr.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < arr.len() {
                if let Some(x) = arr[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(x)));
                    curr.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        root2
    }
    #[test]
    fn test1() {
        let mut org_tree = self::array_to_tree(&[Some(1), Some(3), None, None, Some(2)]);
        let ans_tree = self::array_to_tree(&[Some(3), Some(1), None, None, Some(2)]);

        Solution::recover_tree(&mut org_tree);

        assert_eq!(org_tree, ans_tree);
    }

    #[test]
    fn test2() {
        let mut org_tree = self::array_to_tree(&[Some(3), Some(1), Some(4), None, None, Some(2)]);
        let ans_tree = self::array_to_tree(&[Some(2), Some(1), Some(4), None, None, Some(3)]);

        Solution::recover_tree(&mut org_tree);

        assert_eq!(org_tree, ans_tree);
    }
}
