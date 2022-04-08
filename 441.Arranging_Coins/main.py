# Can be solved as a Quadratic Equation -> (Next Attempt)

def Solution:
    def arrangeCoins(self, n: int) -> int:
        temp = 0
        i = 0
        while temp <= n:
            i += 1
            temp += i
        return i - 1
