"""
You're given strings J representing the types of stones that are jewels, and S representing the stones you have.  Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.

The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive, so "a" is considered a different type of stone from "A".

Example 1:

Input: J = "aA", S = "aAAbbbb"
Output: 3

Example 2:

Input: J = "z", S = "ZZ"
Output: 0

Note:

    S and J will consist of letters and have length at most 50.
    The characters in J are distinct.


"""

class Solution:
    def numJewelsInStones_1(self, J: str, S: str) -> int:
        buffer = { x: 0 for x in J }
        for char in S:
            if char in J:
                buffer[char] += 1
        return sum(x for x in buffer.values())

    def numJewelsInStones(self, J: str, S: str) -> int:
        count: int = 0
        for x in S:
            if x in J:
                count += 1
        return count

if __name__ == "__main__":
    print(Solution().numJewelsInStones('aA', 'aAAbbbb'))