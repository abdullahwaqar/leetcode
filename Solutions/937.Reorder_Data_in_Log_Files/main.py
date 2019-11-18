class Solution:
    def reorderLogFiles(self, logs: List[str]) -> List[str]:
        """
        :type logs: List[str]
        :rtype: List[str]
        """
        digits = []
        letters = []
		# divide logs into two parts, one is digit logs, the other is letter logs
        for log in logs:
            if log.split()[1].isdigit():
                digits.append(log)
            else:
                letters.append(log)

        letters.sort(key=lambda log: log.split()[0])            #when suffix is tie, sort by identifier
        letters.sort(key=lambda log: log.split()[1:])           #sort by suffix
        result = letters + digits                                        #put digit logs after letter logs
        return result


if __name__ == "__main__":
    Solution().reorderLogFiles(["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit dig","let3 art zero"])