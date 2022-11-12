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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if p.is_none() && q.is_none() {
                return true;
            }

            if p.is_none() || q.is_none() {
                return false;
            }
            let p = p.unwrap();
            let q = q.unwrap();
            if p.borrow().val != q.borrow().val {
                return false;
            }

            return dfs(p.borrow().left.clone(), q.borrow().right.clone())
                && dfs(p.borrow().right.clone(), q.borrow().left.clone());
        }

        if let Some(r) = root {
            let r = r.borrow();
            return dfs(r.left.clone(), r.right.clone());
        }

        return true;
    }
}

fn main() {
    println!("Hello, world!");
}
