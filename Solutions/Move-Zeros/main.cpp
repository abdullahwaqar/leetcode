#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    void moveZeroes(std::vector<int>& nums) {
        int zero_count = 0;
        int size = nums.size();

        for (int n : nums) {
            if (n != 0) {
                nums[zero_count] = n;
                ++zero_count;
            }
        }

        for (int i = zero_count; i < size; ++i)
            nums[i] = 0;

        std::for_each(nums.begin(), nums.end(), [&](int el) {
            std::cout << el << '-' << zero_count << std::endl;
        });
    }
};

int main(int argc, char const *argv[]) {
    // std::vector<int> test{0,1,0,3,12};
    // Solution().moveZeroes(test);

    std::vector<int> test1{0, 0, 1};
    Solution().moveZeroes(test1);

    std::cout << std::endl << std::endl << std::endl;

    std::vector<int> test2{0,1,0,3,12};
    Solution().moveZeroes(test2);
    return 0;
}
