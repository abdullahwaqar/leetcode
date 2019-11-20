from typing import List

class Solution:
    def missingNumber_(self, nums: List[int]) -> int:
        num: List = sorted(nums)
        min_num: int = num[0]
        max_num: int = num[len(num) -1]
        while min_num != max_num:
            if num[min_num] != min_num:
                return min_num
            min_num += 1

    def missingNumber(self, nums: List[int]) -> int:
        return sum(range(len(nums)+1)) - sum(nums)

if __name__ == "__main__":
    Solution().missingNumber([9,6,4,2,3,5,7,0,1])