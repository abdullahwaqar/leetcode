#include <algorithm>
#include <iostream>
#include <vector>

class Solution {

  public:
    int minCostToMoveChips(std::vector<int> &chips) {
        int even = 0;
        int odd = 0;
        std::for_each(chips.begin(), chips.end(), [&](int &chip) {
            if (chip % 2 == 0) {
                ++even;
            } else {
                ++odd;
            }
        });
        return std::min(even, odd);
    }
};

int main(int argc, char const *argv[]) {
    auto test = std::vector<int>{2, 2, 2, 3, 3};
    auto s = Solution().minCostToMoveChips(test);
    std::cout << s << std::endl;
    return 0;
}
