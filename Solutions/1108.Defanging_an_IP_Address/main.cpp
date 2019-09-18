/*
Given a valid (IPv4) IP address, return a defanged version of that IP address.

A defanged IP address replaces every period "." with "[.]".



Example 1:

Input: address = "1.1.1.1"
Output: "1[.]1[.]1[.]1"

Example 2:

Input: address = "255.100.50.0"
Output: "255[.]100[.]50[.]0"



Constraints:

    The given address is a valid IPv4 address.

*/

#include <iostream>

class Solution {
public:
    std::string defangIPaddr(std::string address) {
        std::string result;
        for (int i = 0; i < address.length(); ++i) {
            if (address[i] == '.') {
                result += "[.]";
            } else {
                result += address[i];
            }
        }
        return result;
    }
};

int main(int argc, char const *argv[]) {
    std::string sol = Solution().defangIPaddr("255.100.50.0");
    std::cout << sol;
    return 0;
}
