#include <algorithm>
#include <iostream>
#include <map>

class Solution {
  public:
    int firstUniqChar(std::string s) {
        std::map<char, int> counter;
        for (const char c : s) {
            if (counter[c]) {
                ++counter[c];
            } else {
                counter[c] = 1;
            }
        }

        std::for_each(counter.begin(), counter.end(), [&]() {

        });
    }
};

int main(int argc, char const *argv[]) {
    Solution().firstUniqChar()
    return 0;
}
