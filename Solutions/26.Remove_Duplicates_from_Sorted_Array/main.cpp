#include <algorithm>
#include <vector>

class Solution {
  public:
    int removeDuplicates(std::vector<int> &nums) {
        if (nums.empty()) {
            return 0;
        }

        int i = 0;
        for (int j = 1; j < nums.size(); j++) {
            // * Move the poiinter only when there is non equal numbers
            if (nums[j] != nums[i]) {
                // * Increment pointer before swapping
                ++i;
                std::swap(nums[i], nums[j]);
            }
        }
        return i + 1;
    }
};