#include <iostream>
#include <map>
#include <vector>

class Solution {
  public:
    bool containsNearbyAlmostDuplicate(std::vector<int> &nums, int k, int t) {
        if (nums.size() < 2 || k < 0 || t < 0) {
            return false;
        }

        // for (int i = 0; i < nums.size(); ++i) {
        //     if (i > k)
        //         s.erase(s.find(nums[i - k - 1]));

        //     auto it = s.insert(nums[i]);
        //     if (it != begin(s) && *it - *prev(it) <= t)
        //         return true;
        //     if (next(it) != end(s) && *next(it) - *it <= t)
        //         return true;
        // }
        // return false;

        std::map<long long, int> set;
        int curr = 0;
        for (int i = 0; i < nums.size(); ++i) {
            if (i - curr > k) {
                set.erase(nums[curr++]);
            }
            auto floor = set.lower_bound((long long)nums[i] - t);
            if (floor != set.end() && abs(floor->first - nums[i]) <= t)
                return true;
            set[nums[i]] = i;
        }
        return false;
    }
};