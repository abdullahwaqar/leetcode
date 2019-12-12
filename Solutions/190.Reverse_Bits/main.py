class Solution:
    def reverseBits(self, n: int) -> int:
        return int(bin(n)[2:].zfill(32)[::-1], 2)

if __name__ == "__main__":
    Solution().reverseBits(11111111111111111111111111111101)