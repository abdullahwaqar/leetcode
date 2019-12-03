from typing import Dict

class Solution:
    def romanToInt(self, s: str) -> int:
        symbols: Dict = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000
        }
        result = 0
        for i in range(len(s) - 1):
            if symbols[s[i]] < symbols[s[i + 1]]:
                result -= symbols[s[i]]
            else:
                result += symbols[s[i]]
        return result + symbols[s[-1]]

if __name__ == "__main__":
    print(Solution().romanToInt("IV"))
    print(Solution().romanToInt("LVIII"))
    print(Solution().romanToInt("MCMXCIV"))
    print(Solution().romanToInt("MMXIX"))