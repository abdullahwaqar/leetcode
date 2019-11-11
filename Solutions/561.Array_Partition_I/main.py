#  Given an array of 2n integers, your task is to group these integers into n pairs of integer, say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n as large as possible. `
class Solution:
    def arrayPairSum(self, nums: list[int]) -> int:
        return self.max_pair_sum(nums)

    def max_pair_sum(data: list):
        return sum(i for i in sorted(data)[::2])
