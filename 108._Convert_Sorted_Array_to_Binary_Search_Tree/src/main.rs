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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursion_go_recursion_brrrr(
            nums: &Vec<i32>,
            left: i32,
            right: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right {
                return None;
            }

            let mid = (left + right) / 2;
            let mut root = TreeNode::new(nums[mid as usize]);
            root.left = recursion_go_recursion_brrrr(nums, left, mid - 1);
            root.right = recursion_go_recursion_brrrr(nums, mid + 1, right);

            return  Some(Rc::new(RefCell::new(root)));
        }

        return recursion_go_recursion_brrrr(&nums, 0, nums.len() as i32 - 1);
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
    );
}
