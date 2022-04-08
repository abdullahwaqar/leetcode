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
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.clone();

        while ptr.is_some() && ptr.as_ref()?.next.is_some() {
            ptr = ptr.unwrap().next.unwrap().next;
            head = head.unwrap().next
        }

        return head;
    }
}

fn main() {
    println!("Hello, world!");
}
