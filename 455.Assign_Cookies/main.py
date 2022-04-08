class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        greed = sorted(g) # sort the array
        cookies = sorted(s) # sorting cookies
        count = 0
        for i in range(len(cookies)):
            if greed: # Run as long as there are elements in greed
                if cookies[i] >= greed[0]: # if cookie weight is equal or greater than hunger ++count / only comparing to the first element of the greed array
                    count += 1
                    greed.remove(greed[0]) # remove the element
        return count