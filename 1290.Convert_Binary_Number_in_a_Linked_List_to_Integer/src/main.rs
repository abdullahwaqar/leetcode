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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut curr_head = &head;
        let mut result = 0;

        while let Some(node) = curr_head {
            result *= 2;
            result += node.val;
            curr_head = &node.next;
        }

        return result;
    }
}

fn main() {
    println!("Hello, world!");
}
