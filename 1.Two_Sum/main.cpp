/*
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/

#include <vector>
#include <iostream>

class Solution {
public:
    std::vector<int> twoSum(std::vector<int>& nums, int target) {
        std::vector<int> ret;
        for (int i = 0; i < nums.size(); ++i) {
            for (int j = i + 1; j < nums.size(); ++j) {
                if (nums[j] == target - nums[i]) {
                    ret = {i, j};
                }
            }
        }
        return ret;
    }
};

int main(int argc, char const *argv[]) {
    std::vector<int> dataset = {2,7,11,15};
    std::vector<int> sol = Solution().twoSum(dataset, 9);
    for (int n : sol) {
        std::cout << n;
    }
    return 0;
}
