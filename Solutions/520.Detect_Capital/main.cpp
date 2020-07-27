#include <algorithm>
#include <iostream>
#include <string>

class Solution {
  public:
    bool detectCapitalUse(std::string word) {
        if (word.length() == 0) {
            return true;
        }

        bool all_cap = (word.at(0) <= 'Z');
        bool all_low = true;

        for (int i = 1; i < word.size(); ++i) {
            if (word.at(i) > 'Z')
                all_cap = false;
            else
                all_low = false;
        }
        return all_cap || all_low;
    }
};

int main(int argc, char const *argv[]) {
    Solution().detectCapitalUse("USE");
    return 0;
}
