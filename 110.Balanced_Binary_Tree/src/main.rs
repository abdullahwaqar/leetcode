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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            let root = match root {
                Some(a) => a,
                None => return (true, 0),
            };

            let (left_valid, left_height) = dfs(root.borrow().left.clone());
            let (right_valid, right_height) = dfs(root.borrow().right.clone());
            let height = 1 + std::cmp::max(left_height, right_height);
            let is_valid = left_valid && right_valid && (left_height - right_height).abs() <= 1;
            return (is_valid, height);
        }
        return dfs(root).0;
    }
}

fn main() {
    println!("Hello, world!");
}
