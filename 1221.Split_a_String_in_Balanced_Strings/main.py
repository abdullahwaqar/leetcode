class Solution:
    def balancedStringSplit(self, s: str) -> int:
        #* Greedy approach
        a, answer = 0, 0
        for i in s:
            a += 1 if i == 'R' else -1
            if a == 0:
                answer += 1
        return answer

if __name__ == "__main__":
    s = 'RLLLLRRRLR'
    print(Solution().balancedStringSplit(s))