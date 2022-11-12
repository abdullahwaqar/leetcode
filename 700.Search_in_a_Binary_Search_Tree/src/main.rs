use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
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

struct Solution {}

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // * Base case
        if let Some(node) = &root {
            if node.borrow().val == val {
                return root;
            } else if node.borrow().val < val {
                return Self::search_bst(node.borrow().right.clone(), val);
            }
            return Self::search_bst(node.borrow().left.clone(), val);
        } else {
            return None;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
