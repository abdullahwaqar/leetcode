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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        
        if let Some(node) = root {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            //match children
            if let Some(left_node) = left {
                let left_node = left_node.borrow();
                if left_node.left.is_none() && left_node.right.is_none() {
                    sum += left_node.val;
                } else {
                    sum += Self::sum_of_left_leaves(left.clone());
                }
            }
            sum += Self::sum_of_left_leaves(right.clone());
        }
        return sum;
    }
}

fn main() {
    println!("Hello, world!");
}
