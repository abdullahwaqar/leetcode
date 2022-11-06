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
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut p_node, mut q_node) = (p.unwrap().borrow().val, q.unwrap().borrow().val);

        while let Some(node) = root.clone() {
            let node = node.borrow();

            if p_node > node.val && q_node > node.val {
                root = node.right.clone();
            } else if p_node < node.val && q_node < node.val {
                root = node.left.clone();
            } else {
                 return root;
            }
        }

        return root;
    }
}

fn main() {
    println!("Hello, world!");
}
