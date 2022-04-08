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

struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| match head.next {
            Some(mut node) => {
                head.next = Solution::swap_pairs(node.next);
                node.next = Some(head);
                node
            }
            None => head,
        })
    }
}

fn main() {
    println!("Hello, world!");
}
