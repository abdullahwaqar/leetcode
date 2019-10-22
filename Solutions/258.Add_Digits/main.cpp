/*
Given a non-negative integer num, repeatedly add all its digits until the result has only one digit.

Example:

Input: 38
Output: 2
Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2.
             Since 2 has only one digit, return it.

*/
#include <iostream>
#include <stdlib.h>

class Solution {
public:
    int addDigits(int num) {
        while(num > 9){
            int res = 0;
            while(num > 0){
                res += num % 10;
                num /= 10;
                // std::cout << num << std::endl;
                std::cout << res << std::endl;
            }
            num = res;
        }
        return num;
    }
};

int main(int argc, char const *argv[]) {
    int sol = Solution().addDigits(199);
    std::cout << sol;
    return 0;
}
