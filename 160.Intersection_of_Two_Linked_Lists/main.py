# Definition for singly-linked list.
from typing import Optional


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def getIntersectionNode(self, headA: ListNode,
                            headB: ListNode) -> Optional[ListNode]:
        h_a, h_b = headA, headB
        while h_a != h_b:
            h_a = headB if not h_a else h_a.next
            h_b = headA if not h_b else h_b.next
        return h_a