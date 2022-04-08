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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
            (Some(mut a), Some(mut b)) => {
                if a.val > b.val {
                    b.next = Self::merge_two_lists(b.next, Some(a));
                    return Some(b);
                } else {
                    a.next = Self::merge_two_lists(a.next, Some(b));
                    return Some(a);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
