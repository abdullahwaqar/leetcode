class Solution:
    def uniqueOccurrences(self, arr) -> bool:
        temp = { i : 0 for i in arr }
        for num in arr:
            if num in arr:
                temp[num] += 1
                if temp[num] > 1:
                    print(temp)
                    return True
        return False


if __name__ == "__main__":
    # print(Solution().uniqueOccurrences([1,2,2,1,1,3]))
    # print(Solution().uniqueOccurrences([1,2]))
    # print(Solution().uniqueOccurrences([-3,0,1,-3,1,1,1,-3,10,0]))
    print(Solution().uniqueOccurrences([3,5,-2,-3,-6,-6]))