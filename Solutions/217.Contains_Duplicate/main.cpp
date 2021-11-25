/*
*Given an integer array nums, return true if any value appears at least twice in
the array, and return false if every element is distinct.



Example 1:

Input: nums = [1,2,3,1]
Output: true

Example 2:

Input: nums = [1,2,3,4]
Output: false

Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true
*/

#include <iostream>
#include <map>
#include <set>
#include <vector>

class Solution {
  public:
    bool containsDuplicate(std::vector<int> &nums) {
        std::map<int, bool> buffer;

        for (auto num : nums) {
            if (!buffer.count(num)) {
                buffer[num] = true;
            } else {
                return true;
            }
        }
        return false;
    }

    bool containsDuplicateSet(std::vector<int> &nums) {
        std::set<int> buffer(nums.begin(), nums.end());
        return buffer.size() < nums.size();
    }
};

int main(int argc, char const *argv[]) {
    std::vector<int> nums = {1, 1, 1, 3, 3, 4, 3, 2, 4, 2};
    auto s = Solution();

    std::cout << s.containsDuplicate(nums);
    return 0;
}
