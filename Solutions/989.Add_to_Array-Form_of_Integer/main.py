class Solution:
    def addToArrayForm(self, A, K):
        strNum = ''
        for x in A:
            strNum += str(x)
        sum = int(strNum) + K
        print(list(str(sum)))

if __name__ == "__main__":
    A = [2,7,4]
    K = 181
    Solution().addToArrayForm(A, K)