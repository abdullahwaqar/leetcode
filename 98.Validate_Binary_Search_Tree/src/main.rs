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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, left_val: Option<i32>, right_val: Option<i32>) -> bool {
            if let Some(node) = node.as_ref() {
                let node = node.borrow();
                let cur_val = node.val;
                (left_val.is_none() || cur_val > left_val.unwrap())
                    && (right_val.is_none() || cur_val < right_val.unwrap())
                    && dfs(node.left.clone(), left_val, Some(cur_val))
                    && dfs(node.right.clone(), Some(cur_val), right_val)
            } else {
                return true;
            }
        }
        return dfs(root, None, None);
    }
}

fn main() {
    let root = Option::Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().left =
        Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right =
        Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!("{:?}", Solution::is_valid_bst(root));
}
