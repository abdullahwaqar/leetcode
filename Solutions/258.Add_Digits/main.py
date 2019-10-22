class Solution:
    def addDigits(self, num: int) -> int:
        num = num
        while num > 9:
            result = 0
            while num > 0:
                result += num % 10
                num // 10
            num = result
        return num

if __name__ == "__main__":
    print(Solution().addDigits(199))
    print(Solution().addDigits(38))