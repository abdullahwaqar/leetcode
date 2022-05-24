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

struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // if p.is_none() && q.is_none() {
        //     return true;
        // }

        // if p.is_none() || q.is_none() {
        //     return false;
        // }

        // return Solution::is_same_tree(
        //     p.as_ref().unwrap().borrow().right.clone(),
        //     q.as_ref().unwrap().borrow().right.clone(),
        // ) && Solution::is_same_tree(
        //     p.unwrap().borrow().left.clone(),
        //     q.unwrap().borrow().left.clone(),
        // );
        match (p, q) {
            (Some(p), Some(q)) => {
                return p.borrow().val == q.borrow().val
                    && Solution::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Solution::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
