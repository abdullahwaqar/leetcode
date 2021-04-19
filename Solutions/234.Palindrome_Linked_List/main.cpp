struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
  public:
    /**
     * In the beginning, set two pointersfastandslowstarting at the head.
     * (1) Move:fastpointer goes to the end, andslowgoes to the middle.
     * (2) Reverse: the right half is reversed, andslowpointer becomes the 2nd
     * head. (3) Compare: run the two pointersheadandslowtogether and compare.
     */
    bool isPalindrome(ListNode *head) {
        if (head == nullptr || head->next == nullptr) {
            return true;
        }

        auto slow = head;
        auto fast = head;

        while (fast != nullptr && fast->next != nullptr) {
            fast = fast->next->next;
            slow = slow->next;
        }
        //* For odd length
        if (fast != nullptr) {
            slow = slow->next;
        }

        //* Slow pointer is at mid
        //* Set fast to head and reverse second half
        fast = head;

        auto prev = reverse(slow);
        while (prev != nullptr) {
            if (head->val != prev->val) {
                return false;
            }
            head = head->next;
            prev = prev->next;
        }
        return true;
    }

    ListNode *reverse(ListNode *head) {
        ListNode *prev = nullptr;
        ListNode *curr = head;

        while (curr != nullptr) {
            ListNode *tmpNext = curr->next;
            curr->next = prev;
            prev = curr;
            curr = tmpNext;
        }
        return prev;
    }
};