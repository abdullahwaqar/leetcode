/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};
class Solution {
  public:
    bool hasCycle(ListNode *head) {
        auto fast_ptr = head;
        auto slow_ptr = head;

        while (fast_ptr != nullptr && fast_ptr->next || nullptr) {
            slow_ptr = slow_ptr->next;
            fast_ptr = fast_ptr->next->next;

            if (slow_ptr == fast_ptr) {
                return true;
            }
        }

        return false;
    }
};

int main(int argc, char const *argv[]) {
    /* code */
    return 0;
}
