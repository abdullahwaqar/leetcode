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

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution {}

impl Solution {
    /**
     * Pre order: NLR - Node Left Right
     */
    pub fn preorder_traversal(root: Node) -> Vec<i32> {
        let mut list_buffer = Vec::new();

        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            let left = &mut Self::preorder_traversal(node.left.clone());
            let right = &mut Self::preorder_traversal(node.right.clone());

            // N
            list_buffer.push(val);
            // L
            list_buffer.append(left);
            // R
            list_buffer.append(right);
        }
        return list_buffer;
    }
}

fn main() {
    println!("Hello, world!");
}
