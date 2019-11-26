from typing import List, Dict

class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        hash_list: Dict = {}
        for num in nums:
            try:
                hash_list.pop(num)
            except:
                hash_list[num] = 1
        return hash_list.popitem()[0]


if __name__ == "__main__":
    print(Solution().singleNumber([4,1,2,1,2]))