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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut buffer = None;
        let mut tail = &mut buffer;

        while let Some(mut node) = head.take() {
            head = node.next.take();

            if node.val != val {
                *tail = Some(node);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }

        buffer
    }
}
fn main() {
    println!("Hello, world!");
}
