// Definition for singly-linked list.
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (head, root) {
                (Some(h), Some(r)) => {
                    dfs(head, root)
                        || helper(head, &r.borrow().left)
                        || helper(head, &r.borrow().right)
                }
                (Some(_), None) => false,
                _ => true,
            }
        }
        fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (head, root) {
                (Some(h), Some(r)) => {
                    h.val == r.borrow().val
                        && (dfs(&h.next, &r.borrow().left) || dfs(&h.next, &r.borrow().right))
                }
                (Some(_), None) => false,
                _ => true,
            }
        }

        return helper(&head, &root);
    }
}

fn main() {
    println!("Hello, world!");
}
