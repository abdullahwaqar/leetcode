/**
 Given an array of integers nums and an integer k, return the number of unique
k-diff pairs in the array.

A k-diff pair is an integer pair (nums[i], nums[j]), where the following are
true:

    0 <= i, j < nums.length
    i != j
    a <= b
    b - a == k

Constraints:

    1 <= nums.length <= 104
    -107 <= nums[i] <= 107
    0 <= k <= 107

*/

#include <algorithm>
#include <vector>

using namespace std;

class Solution {
  public:
    /**
     * Use double pointers. Or first sort the data and define the left and right
        pointers, starting from 0, respectively. If the right and left pointers
     are equal, it means that the first or repeated cycle, you need to move the
     right pointer one bit back. If the element pointed to by the left pointer
     is equal to the element pointed to by the right pointer after adding k,
     then the number of times is increased by 1. Then it is judged that if the
     element after the pointed position of the right pointer is equal to the
     current element, then the right pointer continues to move backwards. If the
     element pointed to by the left pointer is smaller than the element pointed
     to by the right pointer after adding k, it means that the element on the
     left is smaller and the left pointer moves forward. If the element pointed
     by the left pointer is larger than the element pointed by the right pointer
     after adding k, it means that the element on the right is smaller and the
     right pointer moves forward.

        The time complexity and space complexity of this method are O(n log(n))
     and O(1).
     */
    int findPairs(vector<int> &nums, int k) {
        if (nums.size() == 0 || k < 0) {
            return 0;
        }

        sort(nums.begin(), nums.end());

        int count = 0;
        int start = 0;
        int end = 0;

        while (end < nums.size()) {
            if (start == end) {
                ++end;
            } else if (nums[start] + k == nums[end]) {
                count++;
                while (end + 1 < nums.size() && nums[end] == nums[end + 1]) {
                    end++;
                }
                end++;
            } else if (nums[start] + k < nums[end]) {
                start++;
            } else if (nums[start] + k > nums[end]) {
                end++;
            }
        }
        return count;
    }
};